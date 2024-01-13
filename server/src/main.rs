mod controller;

#[macro_use]
extern crate rocket;

use crate::controller::{index, stream_dto, template_command};
use anyhow::{Context, Result};
use chrono_craft_engine::cache_db::redis::RedisStateDb;
use chrono_craft_engine::repository::{DtoRepository, Repository, StateRepository};
use eventstore::Client;
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::Template;
use std::env;
use std::net::Ipv4Addr;
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

type TemplateStateCache = RedisStateDb<TemplateState>;
type TemplateRepository = StateRepository<TemplateState, TemplateStateCache>;
type TemplateDtoCache = RedisStateDb<TemplateDto>;
type TemplateDtoRepository = DtoRepository<TemplateDto, TemplateDtoCache>;

const STREAM_NAME: &str = "template2";

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[rocket::main]
async fn main() -> Result<()> {
    let settings = env::var("EVENTSTORE_URI")
        .context("fail to get EVENTSTORE_URI env var")?
        .parse()
        .context("fail to parse the settings")?;

    let redis_client =
        redis::Client::open(env::var("REDIS_URI").context("fail to get REDIS_URI env var")?)?;

    let event_store_db = Client::new(settings).context("fail to connect to eventstore db")?;

    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(event_store_db, dto_redis.clone());

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
        .merge(("address", Ipv4Addr::new(0, 0, 0, 0)))
        .merge(("template_dir", "server/templates"));
    let _rocket = rocket::custom(figment)
        .manage(repo_state)
        .manage(repo_dto)
        .manage(dto_redis)
        .mount("/", routes![index, redirect_index_js])
        .mount("/api", routes![template_command, stream_dto])
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .attach(Template::fairing())
        .register("/", catchers![general_not_found])
        .launch()
        .await;

    Ok(())
}

#[get("/template/index.js")]
fn redirect_index_js() -> Redirect {
    Redirect::temporary(format!(
        "/template/index-v{}.js",
        built_info::PKG_VERSION.replace('.', "-")
    ))
}

#[catch(404)]
fn general_not_found() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <p>Hmm... This is not the droïd you are looking for, oupsi</p>
    "#,
    )
}
