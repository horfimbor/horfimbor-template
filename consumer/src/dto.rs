use anyhow::Result;
use chrono_craft_engine::repository::Repository;
use chrono_craft_engine::Stream;
use eventstore::Client;
use redis::Client as Redis;

use crate::{GROUP_NAME, STREAM_NAME, TemplateDtoCache, TemplateDtoRepository};

pub async fn cache_dto(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let dto_redis = TemplateDtoCache::new(redis_client.clone());

    let repo_dto = TemplateDtoRepository::new(event_store_db, dto_redis.clone());

    let stream = Stream::Stream(STREAM_NAME);
    repo_dto.cache_dto(&stream, GROUP_NAME).await?;

    Ok(())
}
