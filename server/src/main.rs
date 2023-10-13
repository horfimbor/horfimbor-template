mod controller;

#[macro_use]
extern crate rocket;

use crate::controller::{cached_dto_html, index, stream_dto, template_command};
use anyhow::{Context, Result};
use eventstore::Client;
use gyg_eventsource::cache_db::redis::RedisStateDb;
use gyg_eventsource::repository::{DtoRepository, Repository, StateRepository};
use gyg_eventsource::Stream;
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket::tokio;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::Template;
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

type TemplateStateCache = RedisStateDb<TemplateState>;
type TemplateRepository = StateRepository<TemplateState, TemplateStateCache>;
type TemplateDtoCache = RedisStateDb<TemplateDto>;
type TemplateDtoRepository = DtoRepository<TemplateDto, TemplateDtoCache>;

const STREAM_NAME: &str = "template2";
const GROUP_NAME: &str = "t2";

#[rocket::main]
async fn main() -> Result<()> {
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .to_string()
        .parse()
        .context("fail to parse the settings")?;

    let event_store_db = Client::new(settings).context("fail to connect to eventstore db")?;

    let redis_client = redis::Client::open("redis://localhost:6379/")?;

    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(event_store_db, dto_redis.clone());

    let repo_dto_spawn = repo_dto.clone();
    tokio::spawn(async move {
        let stream = Stream::Stream(STREAM_NAME);
        repo_dto_spawn.cache_dto(&stream, GROUP_NAME).await
    });

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[
            "http://127.0.0.1:8000",
            "http://localhost:8000",
        ]),
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .context("fail to create cors")?;

    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("template_dir", "server/templates"));
    let _rocket = rocket::custom(figment)
        .manage(repo_state)
        .manage(repo_dto)
        .manage(dto_redis)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![template_command, cached_dto_html, stream_dto],
        )
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .attach(Template::fairing())
        .register("/", catchers![general_not_found])
        .launch()
        .await;

    Ok(())
}

#[catch(404)]
fn general_not_found() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <p>Hmm... This is not the droïd you are looking for, oupsi</p>
    "#,
    )
}
