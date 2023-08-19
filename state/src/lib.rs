use gyg_eventsource::{Dto, State};
use serde::{Deserialize, Serialize};
use template_shared::command::TemplateCommand;
use template_shared::error::TemplateError;
use template_shared::event::TemplateEvent;
use template_shared::START_VALUE;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TemplateState {
    value: usize,
}

impl TemplateState {
    pub fn get_value(&self) -> usize {
        self.value
    }
}

impl Default for TemplateState {
    fn default() -> Self {
        TemplateState { value: START_VALUE }
    }
}

impl Dto for TemplateState {
    type Event = TemplateEvent;
    type Error = TemplateError;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            TemplateEvent::Added(i) => self.value += i,
            TemplateEvent::Removed(i) => self.value -= i,
        }
    }
}

impl State for TemplateState {
    type Command = TemplateCommand;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            TemplateCommand::Add(i) => {
                if self.value + i > 3000 {
                    Err(Self::Error::CannotAdd(i))
                } else {
                    Ok(vec![Self::Event::Added(i)])
                }
            }
            TemplateCommand::Reset => {
                if self.value == 0 {
                    Err(Self::Error::AlreadyEmpty)
                } else {
                    Ok(vec![Self::Event::Removed(self.value)])
                }
            }
        }
    }
}
