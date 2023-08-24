mod controller;

#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use eventstore::Client;
use gyg_eventsource::cache_db::redis::RedisStateDb;
use gyg_eventsource::repository::{DtoRepository, Repository, StateRepository};
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket::tokio;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

use crate::controller::{index, cached_dto, template_command};

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

    let redis_client = redis::Client::open("redis://localhost:6379/").unwrap();

    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(event_store_db, dto_redis.clone());

    tokio::spawn(async move {
        let repo_dto = repo_dto.clone();

        repo_dto.listen(STREAM_NAME, GROUP_NAME).await.unwrap();
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

    let figment = rocket::Config::figment().merge(("port", 8000));
    let _rocket = rocket::custom(figment)
        .manage(repo_state)
        .manage(dto_redis)
        .mount("/", routes![index])
        .mount("/api", routes![template_command, cached_dto])
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
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
