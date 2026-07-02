mod consumer;
mod web;

#[macro_use]
extern crate rocket;

use crate::consumer::delay::{handle_delay, resolve_delay};
use crate::consumer::dto::{CACHE_PREFIX, cache_dto};
use crate::consumer::state::cache_state;
use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use horfimbor_callback_recall::database::sqlite::open;
use horfimbor_callback_recall::{SchedulerBuilder, SchedulerListener};
use horfimbor_eventsource::cache_db::redis::StateDb;
use horfimbor_eventsource::repository::{
    DtoRepository, DtoRepositoryConstructor, RepositoryKind, StateRepository,
    StateRepositoryConstructor,
};
use kurrentdb::Client;
use rocket::futures::future::try_join_all;
use rocket::futures::{FutureExt, StreamExt};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::env;
use std::time::Duration;
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

type TemplateStateCache = StateDb<TemplateState>;
type TemplateRepository = StateRepository<TemplateState, TemplateStateCache>;
type TemplateDtoCache = StateDb<TemplateDto>;
type TemplateDtoRepository = DtoRepository<TemplateDto, TemplateDtoCache>;
type Host = String;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
enum Service {
    Web,
    Delay,
    State,
    Dto,
}

const STREAM_NAME: &str = "tpl";

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    real_env: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Cli {
        #[arg(long)]
        hello: String,
    },
    Service {
        #[arg(long)]
        list: Vec<Service>,
    },
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

    let redis_client =
        redis::Client::open(env::var("REDIS_URI").context("fail to get REDIS_URI env var")?)?;

    let event_store_db =
        Client::new(settings).map_err(|e| anyhow!(" cannot connect to eventstore : {e}"))?;

    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(
        event_store_db.clone(),
        dto_redis.clone(),
        RepositoryKind::Dto(CACHE_PREFIX),
    );

    let template_repository = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let db = open("schedule").await.context("cannot create sqlite db")?;

    let mut builder = SchedulerBuilder::new(db, Duration::from_secs(2))
        .await
        .context("cannot create builder")?;

    let event_name = resolve_delay(&template_repository, &mut builder);

    let (emitter, listener) = builder.start();

    match args.command {
        Command::Service { list } => {
            let mut services = Vec::new();

            if list.is_empty() || list.contains(&Service::Web) {
                services.push(
                    web::start_server(repo_state, repo_dto, dto_redis, redis_client.clone())
                        .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::Delay) {
                services.push(handle_delay(event_store_db.clone(), emitter, event_name).boxed());
            }

            services.push(join_error(listener).boxed());

            if list.is_empty() || list.contains(&Service::Dto) {
                services.push(cache_dto(redis_client.clone(), event_store_db.clone()).boxed());
            }

            if list.is_empty() || list.contains(&Service::State) {
                services.push(cache_state(redis_client, event_store_db).boxed());
            }
            let signals = Signals::new([SIGTERM, SIGINT, SIGQUIT])?;

            let signals_task = handle_signals(signals).boxed();
            services.push(signals_task);

            dbg!(services.len());

            try_join_all(services)
                .await
                .map(|_| ())
                .context("some service failed")
        }
        Command::Cli { hello } => {
            println!("hello {hello} !");
            Ok(())
        }
    }
}

async fn join_error(l: SchedulerListener) -> Result<(), anyhow::Error> {
    l.join().await;

    Ok(())
}

async fn handle_signals(mut signals: Signals) -> Result<()> {
    if signals.next().await.is_some() {
        bail!("Exit required")
    }

    Ok(())
}
