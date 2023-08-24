use crate::{TemplateDtoCache, TemplateRepository, STREAM_NAME};
use gyg_eventsource::model_key::ModelKey;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::State;
use template_shared::command::TemplateCommand;
use template_shared::dto::TemplateDto;
use uuid::Uuid;
use gyg_eventsource::cache_db::CacheDb;

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

#[get("/data")]
pub async fn cached_dto(
    dto_redis: &State<TemplateDtoCache>,
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
    let state = dto_redis.get(&key)
        .map_err(|e| e.to_string())?;

    Ok(Json(state.state().clone()))
}

#[get("/")]
pub async fn index() -> RawHtml<&'static str> {
    // <- request handler
    RawHtml(
        r#"<body>
        <gyg-template></gyg-template>
        <gyg-template></gyg-template>
        <gyg-template></gyg-template>
        <script type="module">
        import init, { run } from './template/index.js';
        async function main() {
            await init();
            run();
        }
        main();
        </script>
        </body>"#,
    )
}
