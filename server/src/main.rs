mod controller;

#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use eventstore::Client;
use gyg_eventsource::model_key::ModelKey;
use gyg_eventsource::repository::EventRepository;
use gyg_eventsource::state_db::{StateDb, StateDbError};
use gyg_eventsource::State;
use rocket::fs::{relative, FileServer};
use rocket::response::content::RawHtml;
use std::marker::PhantomData;
use template_state::TemplateState;

use crate::controller::{state, template_command};

type TemplateNoCache = NoCache<TemplateState>;
type TemplateRepository = EventRepository<TemplateNoCache, TemplateState>;

#[rocket::main]
async fn main() -> Result<()> {
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .to_string()
        .parse()
        .context("fail to parse the settings")?;

    let event_store_db = Client::new(settings).context("fail to connect to eventstore db")?;

    let repo = EventRepository::new(event_store_db, TemplateNoCache::new());

    let figment = rocket::Config::figment().merge(("port", 8000));
    let _rocket = rocket::custom(figment)
        .manage(repo)
        .mount("/api", routes![template_command, state])
        .mount("/", FileServer::from(relative!("web")))
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

impl<S> StateDb<S> for NoCache<S>
where
    S: State,
{
    fn get_from_db(&self, _key: &ModelKey) -> Result<Option<String>, StateDbError> {
        Ok(None)
    }

    fn set_in_db(&self, _key: &ModelKey, _state: String) -> Result<(), StateDbError> {
        Ok(())
    }
}
