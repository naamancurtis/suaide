use serde::{Deserialize, Serialize};
use std::convert::From;

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
            Priority::Critical => String::from("Absolultely Critical"),
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

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum Output {
    Terminal,
    Clipboard,
}

impl From<&str> for Output {
    fn from(s: &str) -> Self {
        match s {
            "t" | "terminal" => Output::Terminal,
            "cb" | "clipboard" => Output::Clipboard,
            _ => panic!("wrong str passed to enum"),
        }
    }
}
