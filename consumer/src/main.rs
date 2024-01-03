mod delay;
mod dto;
mod state;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use eventstore::Client;
use chrono_craft_engine::cache_db::redis::RedisStateDb;
use chrono_craft_engine::repository::{DtoRepository, StateRepository};
use std::env;
use template_shared::dto::TemplateDto;
use template_state::TemplateState;

type TemplateStateCache = RedisStateDb<TemplateState>;
type TemplateRepository = StateRepository<TemplateState, TemplateStateCache>;
type TemplateDtoCache = RedisStateDb<TemplateDto>;
type TemplateDtoRepository = DtoRepository<TemplateDto, TemplateDtoCache>;

const STREAM_NAME: &str = "template2";
const GROUP_NAME: &str = "t2";

#[derive(ValueEnum, Clone, Debug)]
enum Consumer {
    Delay,
    Dto,
    State,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    consumer: Consumer,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let settings = env::var("EVENTSTORE_URI")
        .context("fail to get EVENTSTORE_URI env var")?
        .parse()
        .context("fail to parse the settings")?;

    let redis_client =
        redis::Client::open(env::var("REDIS_URI").context("fail to get REDIS_URI env var")?)?;

    let event_store_db = Client::new(settings).context("fail to connect to eventstore db")?;

    match args.consumer {
        Consumer::Delay => {
            delay::compute_delay(redis_client, event_store_db).await?;
        }
        Consumer::Dto => {
            dto::cache_dto(redis_client, event_store_db).await?;
        }
        Consumer::State => {
            state::cache_state(redis_client, event_store_db).await?;
        }
    }

    Ok(())
}
