use crate::error::TemplateError;
use crate::event::TemplateEvent;
use crate::START_VALUE;
use gyg_eventsource::Dto;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TemplateDto {
    last_ten: Vec<TemplateEvent>,
    average: f32,
}

impl Default for TemplateDto {
    fn default() -> Self {
        TemplateDto {
            last_ten: vec![TemplateEvent::Added(START_VALUE)],
            average: START_VALUE as f32,
        }
    }
}

impl Dto for TemplateDto {
    type Event = TemplateEvent;
    type Error = TemplateError;

    fn play_event(&mut self, event: &Self::Event) {
        self.last_ten.push(event.clone());
        if self.last_ten.len() > 10 {
            self.last_ten.remove(0);
        }
        let mut sum = 0;
        for e in self.last_ten.clone() {
            match e {
                TemplateEvent::Added(a) => sum += a,
                TemplateEvent::Removed(r) => sum -= r,
            }
        }
        self.average = sum as f32 / self.last_ten.len() as f32;
    }
}
