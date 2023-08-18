use crate::TemplateRepository;
use gyg_eventsource::model_key::ModelKey;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::State;
use template_shared::command::TemplateCommand;
use uuid::Uuid;

#[post("/", format = "json", data = "<command>")]
pub async fn template_command(
    state_repository: &State<TemplateRepository>,
    cookies: &CookieJar<'_>,
    command: Json<TemplateCommand>,
) -> Result<String, String> {
    let uuid = match cookies.get("uuid") {
        None => {
            return Err("No cookies".to_string());
        }
        Some(crumb) => crumb.to_string(),
    };
    let key = ModelKey::new(STREAM_NAME.to_string(), uuid);
    let state = state_repository.add_command(&key, command.0, None).await;

    match state {
        Ok(s) => serde_json::to_string(&s).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

const STREAM_NAME: &str = "template";

#[get("/data")]
pub async fn state(
    state_repository: &State<TemplateRepository>,
    cookies: &CookieJar<'_>,
) -> Result<String, String> {
    let uuid = match cookies.get("uuid") {
        None => {
            let uuid = Uuid::new_v4().to_string();
            cookies.add(Cookie::new("uuid", uuid.clone()));
            uuid
        }
        Some(crumb) => crumb.to_string(),
    };

    let key = ModelKey::new(STREAM_NAME.to_string(), uuid);
    let state = state_repository.get_model(&key).await;

    match state {
        Ok(s) => serde_json::to_string(&s).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
