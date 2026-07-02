use crate::{Host, TemplateDtoCache, TemplateDtoRepository, TemplateRepository, built_info};
use anyhow::{Context, Error};
use chrono::{DateTime, Local};
use redis::Client as RedisClient;
use rocket::State;
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::response::Redirect;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_dyn_templates::{Template, context};
use std::env;
use uuid::Uuid;

mod api;

pub async fn start_server(
    repo_state: TemplateRepository,
    repo_dto: TemplateDtoRepository,
    dto_cache: TemplateDtoCache,
    dto_redis: RedisClient,
) -> Result<(), Error> {
    let auth_port = env::var("APP_PORT")
        .context("APP_PORT is not defined")?
        .parse::<u16>()
        .context("APP_PORT cannot be parse in u16")?;
    let auth_host = env::var("APP_HOST").context("APP_HOST is not defined")?;
    let host: Host = auth_host.clone();

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
        .manage(dto_cache)
        .mount("/", routes![index, redirect_index_js])
        .mount("/api", api::routes())
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .attach(Template::fairing())
        .register("/", catchers![general_not_found])
        .launch()
        .await;

    Ok(())
}

#[get("/")]
async fn index(host: &State<Host>) -> Template {
    let local: DateTime<Local> = Local::now();

    let id = Uuid::new_v4();

    Template::render(
        "index",
        context! {
            title: format!("Hello, world! {}",local.format("%x %T")),
            endpoint: format!("{host}/api/" ),
            id: id
        },
    )
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
