use crate::{TemplateDtoRepository, TemplateRepository};
use gyg_eventsource::model_key::ModelKey;
use gyg_eventsource::repository::Repository;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::State;
use template_shared::command::TemplateCommand;
use template_shared::dto::TemplateDto;
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
    let key = ModelKey::new(STREAM_NAME.to_string(), uuid);
    state_repository
        .add_command(&key, command.0, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

const STREAM_NAME: &str = "template";

#[get("/data")]
pub async fn state(
    state_repository: &State<TemplateDtoRepository>,
    cookies: &CookieJar<'_>,
) -> Result<Json<TemplateDto>, String> {
    let uuid = match cookies.get("uuid") {
        None => {
            let uuid = Uuid::new_v4().to_string();
            cookies.add(Cookie::new("uuid", uuid.clone()));
            uuid
        }
        Some(crumb) => crumb.to_string(),
    };

    let key = ModelKey::new(STREAM_NAME.to_string(), uuid);
    let state = state_repository
        .get_model(&key)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(state.state().clone()))
}
