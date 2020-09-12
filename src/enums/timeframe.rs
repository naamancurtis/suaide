use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

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
