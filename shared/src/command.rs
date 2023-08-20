#[cfg(feature = "server")]
use gyg_eventsource::gyg_eventsource_derive::Command;
#[cfg(feature = "server")]
use gyg_eventsource::{Command, CommandName};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Command))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateCommand {
    Add(usize),
    Reset,
}
