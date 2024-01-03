use crate::{TemplateDtoCache, TemplateDtoRepository, TemplateRepository, STREAM_NAME};
use chrono::prelude::*;
use chrono_craft_engine::cache_db::CacheDb;
use chrono_craft_engine::metadata::Metadata;
use chrono_craft_engine::model_key::ModelKey;
use chrono_craft_engine::repository::Repository;
use chrono_craft_engine::Stream;
use rocket::http::{Cookie, CookieJar};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use template_shared::command::TemplateCommand;
use template_shared::event::TemplateEvent;
use uuid::Uuid;

#[post("/", format = "json", data = "<command>")]
pub async fn template_command(
    state_repository: &State<TemplateRepository>,
    cookies: &CookieJar<'_>,
    command: Json<TemplateCommand>,
) -> Result<(), String> {
    let uuid = match get_uuid_from_cookies(cookies) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let key = ModelKey::new(STREAM_NAME, uuid);
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn get_uuid_from_cookies(cookies: &CookieJar) -> Result<String, Result<(), String>> {
    let uuid = match cookies.get("uuid") {
        None => {
            return Err(Err("no cookies".to_string()));
        }
        Some(crumb) => crumb.to_string(),
    }
    .split("=")
    .last()
    .unwrap()
    .to_string();
    Ok(uuid)
}

#[get("/data")]
pub async fn stream_dto(
    dto_redis: &State<TemplateDtoCache>,
    dto_repository: &State<TemplateDtoRepository>,
    cookies: &CookieJar<'_>,
) -> Result<EventStream![], String> {
    let uuid = match get_uuid_from_cookies(cookies) {
        Ok(value) => value,
        Err(_) => {
            let uuid = Uuid::new_v4().to_string();
            cookies.add(Cookie::new("uuid", uuid.clone()));
            uuid
        }
    };

    let key = ModelKey::new(STREAM_NAME, uuid);
    let dto = dto_redis
        .get(&key)
        .map_err(|e| e.to_string())
        .map_err(|_| "cannot find the dto".to_string())?;

    let mut subscription = dto_repository
        .get_subscription(Stream::Model(key), dto.position())
        .await;

    Ok(EventStream! {
        yield Event::json(&dto.state());
        loop {
            let event = if let Ok(event) = subscription.next().await{
                event
            }else{
                yield Event::data("cannot get event").event("error");
                break;
            };
            let original_event = event.get_original_event();
            let metadata: Metadata = if let Ok(metadata) =  serde_json::from_slice(original_event.custom_metadata.as_ref()){
                metadata
            }else{
                yield Event::data("cannot get metdata").event("error");
                break;
            };

            if metadata.is_event(){

                match original_event.as_json::<TemplateEvent>(){
                    Ok(event) =>{
                        yield Event::json(&event);
                    },
                    Err(_) => {
                        yield Event::data("cannot get original event").event("error");
                        break;
                    }
                };

            }
        }
    })
}

#[get("/")]
pub async fn index() -> Template {
    let local: DateTime<Local> = Local::now();
    Template::render(
        "index",
        context! {
            title: format!("Hello, world! {}",local.format("%x %T"))
        },
    )
}
