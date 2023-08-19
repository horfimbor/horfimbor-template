use gyg_eventsource::gyg_eventsource_derive::Event;
use gyg_eventsource::{Event, EventName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Event)]
pub enum TemplateEvent {
    Added(usize),
    Removed(usize),
}
