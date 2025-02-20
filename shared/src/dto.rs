#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;

use crate::START_VALUE;
use crate::event::TemplateEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TemplateDto {
    last_ten: Vec<(char, usize)>,
    average: f32,
}

impl Default for TemplateDto {
    fn default() -> Self {
        Self {
            last_ten: vec![('+', START_VALUE)],
            average: START_VALUE as f32,
        }
    }
}

impl TemplateDto {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            last_ten: vec![],
            average: 0.0,
        }
    }
    pub fn play_event(&mut self, event: &TemplateEvent) {
        match event {
            TemplateEvent::Added(nb) => self.last_ten.push(('+', *nb)),
            TemplateEvent::Removed(nb) => self.last_ten.push(('-', *nb)),
            TemplateEvent::Delayed(_) => {}
            TemplateEvent::DelayDone(_) => {}
        };
        if self.last_ten.len() > 10 {
            self.last_ten.remove(0);
        }
        let mut sum: isize = 0;
        for e in self.last_ten.clone() {
            match e.0 {
                '-' => sum -= e.1 as isize,
                _ => sum += e.1 as isize,
            }
        }
        self.average = sum as f32 / self.last_ten.len() as f32;
    }
    #[must_use]
    pub fn last_ten(&self) -> &Vec<(char, usize)> {
        &self.last_ten
    }
    #[must_use]
    pub fn average(&self) -> f32 {
        self.average
    }
}

#[cfg(feature = "server")]
impl Dto for TemplateDto {
    type Event = TemplateEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
