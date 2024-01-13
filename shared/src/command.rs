#[cfg(feature = "server")]
use chrono_craft_engine::chrono_craft_engine_derive::Command;
#[cfg(feature = "server")]
use chrono_craft_engine::{Command, CommandName};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Delay {
    pub delay: usize,
    pub to_add: usize,
}

#[cfg_attr(feature = "server", derive(Command))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateCommand {
    Delayed(Delay),
    Finalize(usize),
    Add(usize),
    Reset,
}
