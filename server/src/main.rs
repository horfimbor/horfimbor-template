mod controller;

#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use eventstore::Client;
use gyg_eventsource::cache_db::{CacheDb, CacheDbError};
use gyg_eventsource::model_key::ModelKey;
use gyg_eventsource::repository::{DtoRepository, Repository, StateRepository};
use gyg_eventsource::{Dto, State};
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::marker::PhantomData;
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

use crate::controller::{state, template_command, index};

type TemplateStateNoCache = NoCache<TemplateState>;
type TemplateRepository = StateRepository<TemplateState, TemplateStateNoCache>;
type TemplateDtoNoCache = DtoNoCache<TemplateDto>;
type TemplateDtoRepository = DtoRepository<TemplateDto, TemplateDtoNoCache>;

#[rocket::main]
async fn main() -> Result<()> {
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .to_string()
        .parse()
        .context("fail to parse the settings")?;

    let event_store_db = Client::new(settings).context("fail to connect to eventstore db")?;

    let repo_state = TemplateRepository::new(event_store_db.clone(), TemplateStateNoCache::new());

    let repo_dto = TemplateDtoRepository::new(event_store_db, TemplateDtoNoCache::new());

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[
            "http://127.0.0.1:8080",
            "http://localhost:8080",
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
        .manage(repo_dto)
        .mount("/", routes![index])
        .mount("/api", routes![template_command, state])
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
        <p>Hmm... This is not the droïd you are looking for</p>
    "#,
    )
}

#[derive(Clone)]
pub struct NoCache<S> {
    state: PhantomData<S>,
}

impl<S> NoCache<S> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }
}

impl<S> Default for NoCache<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> CacheDb<S> for NoCache<S>
where
    S: State,
{
    fn get_from_db(&self, _key: &ModelKey) -> Result<Option<String>, CacheDbError> {
        Ok(None)
    }

    fn set_in_db(&self, _key: &ModelKey, _state: String) -> Result<(), CacheDbError> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct DtoNoCache<S> {
    state: PhantomData<S>,
}

impl<S> DtoNoCache<S> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }
}

impl<S> Default for DtoNoCache<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> CacheDb<S> for DtoNoCache<S>
where
    S: Dto,
{
    fn get_from_db(&self, _key: &ModelKey) -> Result<Option<String>, CacheDbError> {
        Ok(None)
    }

    fn set_in_db(&self, _key: &ModelKey, _state: String) -> Result<(), CacheDbError> {
        Err(CacheDbError::Internal("Not allowed for dto".to_string()))
    }
}
