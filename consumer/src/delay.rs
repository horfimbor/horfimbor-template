use crate::{TemplateRepository, TemplateStateCache};
use anyhow::Result;
use eventstore::{Client, SubscribeToPersistentSubscriptionOptions};
use gyg_eventsource::model_key::ModelKey;
use gyg_eventsource::repository::Repository;
use gyg_eventsource::Stream;
use redis::Client as Redis;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use template_shared::command::TemplateCommand;
use template_shared::event::TemplateEvent;
use tokio::time::sleep;

pub async fn compute_delay(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let repo_state = TemplateRepository::new(
        event_store_db.clone(),
        TemplateStateCache::new(redis_client.clone()),
    );

    let stream = Stream::Event("evt.delayed");
    let group_name = "bob";

    repo_state
        .create_subscription(&stream, group_name)
        .await
        .unwrap();

    let options = SubscribeToPersistentSubscriptionOptions::default().buffer_size(1);

    let mut sub = repo_state
        .event_db()
        .subscribe_to_persistent_subscription(stream.to_string(), group_name, &options)
        .await
        .unwrap();

    loop {
        let repo_state = repo_state.clone();
        let rcv_event = sub.next().await.unwrap();

        let event = rcv_event.event.as_ref().unwrap();

        let json = event.as_json::<TemplateEvent>().unwrap();

        if let TemplateEvent::Delayed(delayed) = json {
            let key = ModelKey::from(event.stream_id.as_str());

            tokio::spawn(async move {
                let now = SystemTime::now();
                let epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

                let to_wait = delayed.timestamp as i64 - epoch as i64;
                dbg!(to_wait);
                if to_wait > 0 {
                    sleep(Duration::from_secs(1) * to_wait as u32).await;
                }

                let s = repo_state
                    .add_command(&key, TemplateCommand::Finalize(delayed.id), None)
                    .await
                    .unwrap();

                dbg!(s);
            });
        }
        sub.ack(rcv_event).await.unwrap();
    }

    Ok(())
}
