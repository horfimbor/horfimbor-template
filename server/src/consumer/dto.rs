use crate::{STREAM_NAME, TemplateDtoCache, TemplateDtoRepository};
use anyhow::Result;
use horfimbor_eventsource::Stream;
use horfimbor_eventsource::repository::Repository;
use kurrentdb::Client;
use redis::Client as Redis;

const GROUP_NAME: &str = "dto";

pub async fn cache_dto(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(event_store_db, dto_redis.clone());

    let stream = Stream::Stream(STREAM_NAME);
    repo_dto.cache_dto(&stream, GROUP_NAME).await?;

    Ok(())
}
