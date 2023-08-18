use gyg_eventsource::gyg_eventsource_derive::Event;
use gyg_eventsource::serde::{Deserialize, Serialize};
use gyg_eventsource::{Event, EventName};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Event)]
pub enum TemplateEvent {
    Added(usize),
    Removed(usize),
}
