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

impl ToString for TemplateEvent {
    fn to_string(&self) -> String {
        match self {
            TemplateEvent::Added(n) => {
                format!("+{}", n)
            }
            TemplateEvent::Removed(n) => {
                format!("-{}", n)
            }
        }
    }
}
