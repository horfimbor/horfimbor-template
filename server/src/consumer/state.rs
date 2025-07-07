use crate::{STREAM_NAME, TemplateRepository, TemplateStateCache};
use anyhow::Result;
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::repository::Repository;
use kurrentdb::Client;
use redis::Client as Redis;

const GROUP_NAME: &str = "state";

pub async fn cache_state(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let dto_redis = TemplateStateCache::new(redis_client.clone());

    let repo_state = TemplateRepository::new(event_store_db, dto_redis.clone());

    let stream = Stream::Stream(STREAM_NAME);
    repo_state.cache_dto(&stream, GROUP_NAME).await?;

    Ok(())
}
