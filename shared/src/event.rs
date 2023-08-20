#[cfg(feature = "server")]
use gyg_eventsource::gyg_eventsource_derive::Event;
#[cfg(feature = "server")]
use gyg_eventsource::{Event, EventName};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateEvent {
    Added(usize),
    Removed(usize),
}
