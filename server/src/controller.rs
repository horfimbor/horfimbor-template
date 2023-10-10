use crate::{TemplateDtoCache, TemplateDtoRepository, TemplateRepository, STREAM_NAME};
use chrono::prelude::*;
use gyg_eventsource::cache_db::CacheDb;
use gyg_eventsource::metadata::Metadata;
use gyg_eventsource::model_key::ModelKey;
use gyg_eventsource::repository::Repository;
use gyg_eventsource::Stream;
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
    let uuid = match cookies.get("uuid") {
        None => {
            return Err("no cookies".to_string());
        }
        Some(crumb) => crumb.to_string(),
    };
    let key = ModelKey::new(STREAM_NAME, uuid);
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[get("/data")]
pub async fn stream_dto(
    dto_redis: &State<TemplateDtoCache>,
    dto_repository: &State<TemplateDtoRepository>,
    cookies: &CookieJar<'_>,
) -> EventStream![] {


    let uuid = match cookies.get("uuid") {
        None => {
            let uuid = Uuid::new_v4().to_string();
            cookies.add(Cookie::new("uuid", uuid.clone()));
            uuid
        }
        Some(crumb) => crumb.to_string(),
    };

    let key = ModelKey::new(STREAM_NAME, uuid);
    let dto = dto_redis.get(&key).map_err(|e| e.to_string()).unwrap();

    let mut subscription = dto_repository
        .get_subscription(Stream::Model(key), dto.position())
        .await;

    EventStream! {
        yield Event::json(&dto.state());
        loop {
            let event = subscription.next().await.unwrap();
            let original_event = event.get_original_event();
            let metadata: Metadata =  serde_json::from_slice(original_event.custom_metadata.as_ref()).unwrap();

            if metadata.is_event(){

                let event = original_event.as_json::<TemplateEvent>().unwrap();

                yield Event::json(&event);
            }
        }
    }
}

#[get("/data.html")]
pub async fn cached_dto_html(
    dto_redis: &State<TemplateDtoCache>,
    cookies: &CookieJar<'_>,
) -> Result<Template, String> {
    let uuid = match cookies.get("uuid") {
        None => {
            let uuid = Uuid::new_v4().to_string();
            cookies.add(Cookie::new("uuid", uuid.clone()));
            uuid
        }
        Some(crumb) => crumb.to_string(),
    };

    let key = ModelKey::new(STREAM_NAME, uuid);
    let dto = dto_redis.get(&key).map_err(|e| e.to_string())?;

    Ok(Template::render(
        "data",
        context! {
            dto: dto.state()
        },
    ))
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
