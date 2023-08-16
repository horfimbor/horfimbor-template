use std::fmt::{Display, Formatter};
use gyg_eventsource::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateError{
    AlreadyEmpty,
    CannotAdd(usize),
}

impl Display for TemplateError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::AlreadyEmpty => {write!(f,"cannot empty an empty state")}
            TemplateError::CannotAdd(n) => {
                write!(f,"cannot add {}", n)
            }
        }
    }
}