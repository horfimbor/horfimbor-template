#[macro_use]
extern crate rocket;

use std::marker::PhantomData;
use gyg_eventsource::model_key::ModelKey;
use gyg_eventsource::repository::EventRepository;
use gyg_eventsource::State;
use gyg_eventsource::state_db::{StateDb, StateDbError};
use rocket::fs::{relative, FileServer};

type EasyNoCache = NoCache<TemplateState>;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("port", 8000));


    let repo = EventRepository::new(get_event_db(), EasyNoCache::new());

    rocket::custom(figment)
        .manage(repo)
        .mount("/", FileServer::from(relative!("web")))
        .register("/", catchers![general_not_found])
}



#[catch(404)]
fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <p>Hmm... This is not the droïd you are looking for</p>
    "#,
    )
}

fn get_event_db() -> Client {
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .to_string()
        .parse()
        .unwrap();
    EventClient::new(settings).unwrap()
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