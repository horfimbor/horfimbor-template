#[cfg(feature = "server")]
use chrono_craft_engine::{StateName, StateNamed};
#[cfg(feature = "server")]
use chrono_craft_engine::chrono_craft_engine_derive::StateNamed;

pub mod command;
pub mod dto;
pub mod error;
pub mod event;


pub const START_VALUE: usize = 1337;

#[cfg_attr(feature = "server", derive(StateNamed))]
pub struct TemplateState {

}