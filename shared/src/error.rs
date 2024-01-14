use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TemplateError {
    AlreadyEmpty,
    CannotAdd(usize),
    DelayOutOfBound(usize),
    CannotCalculateTime,
    DelayNotFound,
}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::AlreadyEmpty => {
                write!(f, "cannot empty an empty state")
            }
            TemplateError::CannotAdd(n) => {
                write!(f, "cannot add {}", n)
            }
            TemplateError::DelayOutOfBound(delay) => {
                write!(f, "cannot wait {} seconds", delay)
            }
            TemplateError::CannotCalculateTime => {
                write!(f, "error calculating time")
            }
            TemplateError::DelayNotFound => {
                write!(f, "delay not found")
            }
        }
    }
}
