use gyg_eventsource::gyg_eventsource_derive::Command;
use gyg_eventsource::serde::{Deserialize, Serialize};
use gyg_eventsource::{Command, CommandName};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Command)]
pub enum TemplateCommand {
    Add(usize),
    Reset,
}
