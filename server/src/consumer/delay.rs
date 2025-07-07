use crate::{TemplateRepository, TemplateStateCache};
use anyhow::{Context, Error, Result};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use redis::Client as Redis;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use template_shared::command::TemplateCommand;
use template_shared::event::{Delayed, TemplateEvent};
use tokio::time::sleep;

pub async fn compute_delay(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let e = TemplateEvent::Delayed(Delayed {
        id: 0,
        timestamp: 0,
        to_add: 0,
    });
    let stream = Stream::Event(e.event_name());
    let group_name = "bob";

    create_subscription(&event_store_db, &stream, group_name)
        .await
        .context("cannot create subscription")?;

    let options = SubscribeToPersistentSubscriptionOptions::default().buffer_size(1);

    let mut sub = repo_state
        .event_db()
        .subscribe_to_persistent_subscription(stream.to_string(), group_name, &options)
        .await
        .context("cannot subscribe")?;

    loop {
        let repo_state = repo_state.clone();
        let rcv_event = sub.next().await.context("cannot get next event")?;

        let event = rcv_event.event.as_ref().context("cannot extract event")?;

        let json = event
            .as_json::<TemplateEvent>()
            .context("cannot extract json")?;

        if let TemplateEvent::Delayed(delayed) = json {
            let key = ModelKey::try_from(event.stream_id())
                .context("cannot convert streamId to ModelKey")?;

            tokio::spawn(async move {
                let now = SystemTime::now();
                let epoch = now
                    .duration_since(UNIX_EPOCH)
                    .context("cannot get timestamp")?
                    .as_secs();

                let to_wait = delayed.timestamp as i64 - epoch as i64;
                dbg!(to_wait);
                if to_wait > 0 {
                    sleep(Duration::from_secs(1) * to_wait as u32).await;
                }

                let s = repo_state
                    .add_command(&key, TemplateCommand::Finalize(delayed.id), None)
                    .await
                    .context("cannot add command")?;

                dbg!(s);

                Ok::<(), Error>(())
            });
        }
        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
