use crate::consumer::dto::CACHE_PREFIX;
use crate::{STREAM_NAME, TemplateDtoCache, TemplateDtoRepository, TemplateRepository};
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::cache_db::CacheDb;
use horfimbor_eventsource::helper::get_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::{Route, State};
use template_shared::command::TemplateCommand;
use template_shared::event::TemplateEvent;
use uuid::Uuid;

pub fn routes() -> Vec<Route> {
    routes![template_command, stream_dto]
}

#[post("/input/<id>", format = "json", data = "<command>")]
async fn template_command(
    state_repository: &State<TemplateRepository>,
    command: Json<TemplateCommand>,
    id: &str,
) -> Result<(), String> {
    let uuid: Uuid = id.parse().map_err(|e: uuid::Error| e.to_string())?;

    let key = ModelKey::new(STREAM_NAME, uuid);
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[get("/data/<id>")]
async fn stream_dto(
    dto_redis: &State<TemplateDtoCache>,
    dto_repository: &State<TemplateDtoRepository>,
    id: &str,
) -> Result<EventStream![], String> {
    let uuid: Uuid = id.parse().map_err(|e: uuid::Error| e.to_string())?;

    let key = ModelKey::new(STREAM_NAME, uuid);
    let dto = dto_redis
        .get(Some(CACHE_PREFIX), &key)
        // .get(None, &key)
        .map_err(|e| format!("cannot find the dto{e}"))?;

    dbg!(&dto);

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
