#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName, StateNamed};

use serde::{Deserialize, Serialize};

use crate::TemplateState;


#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Delay {
    pub delay: usize,
    pub to_add: usize,
}

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(TemplateState))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateCommand {
    Delayed(Delay),
    Finalize(usize),
    Add(usize),
    Reset,
}
