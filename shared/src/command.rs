use gyg_eventsource::gyg_eventsource_derive::Command;
use gyg_eventsource::{Command, CommandName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Command)]
pub enum TemplateCommand {
    Add(usize),
    Reset,
}
