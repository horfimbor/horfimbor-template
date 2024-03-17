use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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
            Self::AlreadyEmpty => {
                write!(f, "cannot empty an empty state")
            }
            Self::CannotAdd(n) => {
                write!(f, "cannot add {n}")
            }
            Self::DelayOutOfBound(delay) => {
                write!(f, "cannot wait {delay} seconds")
            }
            Self::CannotCalculateTime => {
                write!(f, "error calculating time")
            }
            Self::DelayNotFound => {
                write!(f, "delay not found")
            }
        }
    }
}
