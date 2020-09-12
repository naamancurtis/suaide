use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash, Copy, Clone)]
pub enum Priority {
    Critical,
    Today,
    Week,
    Sometime,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Critical => String::from("Absolutely Critical"),
            Priority::Today => String::from("Needs to be done today"),
            Priority::Week => String::from("Needs to be done sometime this week"),
            Priority::Sometime => String::from("Sometime in the future"),
        }
    }
}

impl From<&str> for Priority {
    fn from(s: &str) -> Self {
        match s {
            "t" | "today" => Priority::Today,
            "c" | "critical" => Priority::Critical,
            "w" | "week" => Priority::Week,
            "f" | "future" => Priority::Sometime,
            _ => panic!("incorrect str passed to enum"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash, Copy, Clone)]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            // Status::Open => "Open".truecolor(152, 195, 121),
            Status::Open => "Open".green(),
            Status::InProgress => "In Progress".blue(),
            // Status::InProgress => "In Progress".truecolor(97, 175, 239),
            // Status::Closed => "Completed".truecolor(243, 147, 140),
            Status::Closed => "Completed".yellow(),
        };
        write!(f, "{}", text.bold())
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash, Copy, Clone)]
pub enum Timeframe {
    Today,
    Yesterday,
    Week,
    LastWeek,
    Month,
}

impl From<&str> for Timeframe {
    fn from(s: &str) -> Self {
        match s {
            "today" => Timeframe::Today,
            "yesterday" => Timeframe::Yesterday,
            "week" => Timeframe::Week,
            "last-week" => Timeframe::LastWeek,
            "month" => Timeframe::Month,
            _ => panic!("unable to convert argument to timeframe"),
        }
    }
}
