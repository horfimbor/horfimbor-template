use gyg_eventsource::serde::{Deserialize, Serialize};
use gyg_eventsource::{State, StateName};
use template_shared::command::TemplateCommand;
use template_shared::error::TemplateError;
use template_shared::event::TemplateEvent;

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
        TemplateState { value: 1337 }
    }
}

impl State for TemplateState {
    type Event = TemplateEvent;
    type Command = TemplateCommand;
    type Error = TemplateError;

    fn name_prefix() -> StateName {
        "Template"
    }

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            TemplateEvent::Added(i) => self.value += i,
            TemplateEvent::Removed(i) => self.value -= i,
        }
    }

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
