#[cfg(feature = "server")]
use chrono_craft_engine::{Command, CommandName, StateNamed};
#[cfg(feature = "server")]
use chrono_craft_engine::chrono_craft_engine_derive::Command;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
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
