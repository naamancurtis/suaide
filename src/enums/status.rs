use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash, Copy, Clone)]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

impl From<i16> for Status {
    fn from(i: i16) -> Self {
        match i {
            0 => Status::Open,
            1 => Status::InProgress,
            2 => Status::Closed,
            _ => panic!("Invalid status"),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Status::Open => "Open".green(),
            Status::InProgress => "In Progress".blue(),
            Status::Closed => "Completed".yellow(),
        };
        write!(f, "{}", text.bold())
    }
}
