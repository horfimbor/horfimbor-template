#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use std::fmt::Display;

#[cfg(feature = "server")]
use crate::TEMPLATE_STATE_NAME;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Delayed {
    pub id: usize,
    pub timestamp: u64,
    pub to_add: usize,
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(TEMPLATE_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateEvent {
    Added(usize),
    Removed(usize),
    Delayed(Delayed),
    DelayDone(usize),
}

impl Display for TemplateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Added(n) => {
                format!("+{n}")
            }
            Self::Removed(n) => {
                format!("-{n}")
            }
            Self::Delayed(_) => "~~~".to_string(),
            Self::DelayDone(_) => "~!~".to_string(),
        };
        write!(f, "{str}")
    }
}
