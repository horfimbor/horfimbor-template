use crate::{Host, STREAM_NAME, TemplateDtoCache, TemplateDtoRepository, TemplateRepository};
use chrono::prelude::*;
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::cache_db::CacheDb;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use rocket::State;
use rocket::http::{Cookie, CookieJar};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};
use template_shared::command::TemplateCommand;
use template_shared::event::TemplateEvent;
use uuid::Uuid;

#[post("/", format = "json", data = "<command>")]
pub async fn template_command(
    state_repository: &State<TemplateRepository>,
    cookies: &CookieJar<'_>,
    command: Json<TemplateCommand>,
) -> Result<(), String> {
    let uuid = get_uuid_from_cookies(cookies)?;

    let key = ModelKey::new(
        STREAM_NAME,
        uuid.parse().map_err(|e: uuid::Error| e.to_string())?,
    );
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn get_uuid_from_cookies(cookies: &CookieJar) -> Result<String, String> {
    let uuid = match cookies.get("uuid") {
        None => {
            return Err("no cookies".to_string());
        }
        Some(crumb) => crumb.to_string(),
    }
    .split('=')
    .next_back()
    .ok_or("invalid cookie".to_string())?
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
        Ok(value) => value.parse().map_err(|e: uuid::Error| e.to_string())?,
        Err(_) => {
            let uuid = Uuid::new_v4();
            cookies.add(Cookie::new("uuid", uuid.to_string()));
            uuid
        }
    };

    let key = ModelKey::new(STREAM_NAME, uuid);
    let dto = dto_redis
        .get(&key)
        .map_err(|e| e.to_string())
        .map_err(|_| "cannot find the dto".to_string())?;

    let mut subscription = get_subscription(
        dto_repository.event_db(),
        &Stream::Model(key),
        dto.position(),
    )
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
pub async fn index(host: &State<Host>) -> Template {
    let local: DateTime<Local> = Local::now();
    Template::render(
        "index",
        context! {
            title: format!("Hello, world! {}",local.format("%x %T")),
            endpoint: format!("{host}/api/" )
        },
    )
}
