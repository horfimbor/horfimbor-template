
use gyg_eventsource::{Event, EventName};
use gyg_eventsource::serde::{Deserialize, Serialize};
use gyg_eventsource::gyg_eventsource_derive::Event;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Event)]
pub enum TemplateEvent{
    Added(usize),
    Removed(usize),
}