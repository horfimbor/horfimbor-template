

#[cfg(feature = "server")]
use horfimbor_eventsource::{StateName, StateNamed};

pub mod command;
pub mod dto;
pub mod error;
pub mod event;

pub const START_VALUE: usize = 1337;

pub const TEMPLATE_STATE_NAME: &str = "TMPL_NAME";


struct TemplateState{}

#[cfg(feature = "server")]
impl StateNamed for TemplateState {
    fn state_name() -> StateName {
        TEMPLATE_STATE_NAME
    }
}