mod controller;

#[macro_use]
extern crate rocket;

use crate::controller::{index, stream_dto, template_command};
use anyhow::{Context, Result};
use clap::Parser;
use eventstore::Client;
use horfimbor_eventsource::cache_db::redis::StateDb;
use horfimbor_eventsource::repository::{DtoRepository, Repository, StateRepository};
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::Template;
use std::env;
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

type TemplateStateCache = StateDb<TemplateState>;
type TemplateRepository = StateRepository<TemplateState, TemplateStateCache>;
type TemplateDtoCache = StateDb<TemplateDto>;
type TemplateDtoRepository = DtoRepository<TemplateDto, TemplateDtoCache>;
type Host = String;

const STREAM_NAME: &str = "template2";

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    real_env: bool,
}

#[rocket::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !args.real_env {
        dotenvy::dotenv().context("cannot get env")?;
    }

    let settings = env::var("EVENTSTORE_URI")
        .context("fail to get EVENTSTORE_URI env var")?
        .parse()
        .context("fail to parse the settings")?;

    let auth_port = env::var("APP_PORT")
        .context("APP_PORT is not defined")?
        .parse::<u16>()
        .context("APP_PORT cannot be parse in u16")?;
    let auth_host = env::var("APP_HOST").context("APP_HOST is not defined")?;
    let host: Host = auth_host.clone();

    let redis_client =
        redis::Client::open(env::var("REDIS_URI").context("fail to get REDIS_URI env var")?)?;

    let event_store_db = Client::new(settings).context("fail to connect to eventstore db")?;

    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(event_store_db, dto_redis.clone());

    let allowed_origins = AllowedOrigins::some_exact(&[auth_host]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
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
        .merge(("port", auth_port))
        .merge(("address", "0.0.0.0"))
        .merge(("template_dir", "server/templates"));
    let _rocket = rocket::custom(figment)
        .manage(repo_state)
        .manage(repo_dto)
        .manage(dto_redis)
        .manage(host)
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
        r"
        <p>Hmm... This is not the droïd you are looking for, oupsi</p>
    ",
    )
}
