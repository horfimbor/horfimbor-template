use crate::TemplateStateCache;
use anyhow::{Context, Result, bail};
use chrono::DateTime;
use horfimbor_callback_recall::database::{CallBack, Pool};
use horfimbor_callback_recall::{SchedulerBuilder, SchedulerEmitter};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::StateRepository;
use horfimbor_eventsource::{Event, EventName, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use template_shared::command::TemplateCommand;
use template_shared::event::{Delayed, TemplateEvent};
use template_state::TemplateState;

pub async fn handle_delay<P>(
    event_store_db: Client,
    emitter: SchedulerEmitter<P>,
    event_name: EventName,
) -> Result<()>
where
    P: Pool,
{
    let stream = Stream::Event(event_name);
    let group_name = "template_delay_group";

    create_subscription(&event_store_db, &stream, group_name)
        .await
        .context("cannot create subscription")?;

    let options = SubscribeToPersistentSubscriptionOptions::default().buffer_size(1);

    let mut sub = event_store_db
        .subscribe_to_persistent_subscription(stream.to_string(), group_name, &options)
        .await
        .context("cannot subscribe")?;

    loop {
        let rcv_event = sub.next().await.context("cannot get next event")?;

        let event = rcv_event.event.as_ref().context("cannot extract event")?;

        let json = event
            .as_json::<TemplateEvent>()
            .context("cannot extract json")?;

        let model_key =
            ModelKey::try_from(event.stream_id()).context("cannot convert streamId to ModelKey")?;

        if let TemplateEvent::Delayed(delayed) = json {
            let Some(date) = DateTime::from_timestamp(delayed.timestamp as i64, 0) else {
                bail!("cannot create timestamp");
            };

            emitter
                .schedule(CallBack::new(
                    event_name.to_string(),
                    Vec::from(format!("{model_key}:{}", delayed.to_add)),
                    date,
                ))
                .await
                .context("cannot schedule event")?;
        }
        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}

pub fn resolve_delay<P: Pool>(
    repository: &StateRepository<TemplateState, TemplateStateCache>,
    builder: &mut SchedulerBuilder<P>,
) -> EventName {
    let event_name = TemplateEvent::Delayed(Delayed {
        id: 0,
        timestamp: 0,
        to_add: 0,
    })
    .event_name();

    let repository = repository.clone();

    builder.register(event_name, move |payload| {
        let repository = repository.clone();
        async move {
            let data = String::from_utf8(payload).map_err(|e| e.to_string())?;

            let Some((model_key, value)) = data.split_once(":") else {
                return Err(format!("cannot split: {data}"));
            };

            let model_key =
                ModelKey::try_from(model_key).map_err(|e| format!("bad model_key: {}", e))?;

            let to_add = value
                .parse::<usize>()
                .map_err(|e| format!("bad to add value {}", e))?;

            let _s = repository
                .clone()
                .add_command(&model_key, TemplateCommand::Add(to_add), None)
                .await
                .map_err(|e| format!("cannot add command: {}", e))?;

            Ok(())
        }
    });

    event_name
}
