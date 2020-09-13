use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash, Copy, Clone)]
pub enum Status {
    Open,
    InProgress,
    Closed,
    Cancelled,
}

impl From<i16> for Status {
    fn from(i: i16) -> Self {
        match i {
            0 => Status::Open,
            1 => Status::InProgress,
            2 => Status::Closed,
            3 => Status::Cancelled,
            _ => panic!("Invalid status"),
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "open" | "o" => Status::Open,
            "in-progress" | "inprogress" | "progress" | "ip" => Status::InProgress,
            "closed" | "close" | "c" => Status::Closed,
            "cancel" | "cancelled" => Status::Cancelled,
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
            Status::Cancelled => "Cancelled".red().italic(),
        };
        write!(f, "{}", text.bold())
    }
}
