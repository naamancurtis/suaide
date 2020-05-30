use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    message: String,
    priority: Priority,
    issue_number: Option<String>,
    opened: NaiveDateTime,
    closed: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(id: u32, priority: Priority, message: String, issue_number: Option<String>) -> Self {
        Self {
            id,
            message,
            priority,
            issue_number,
            opened: Utc::now().naive_utc(),
            closed: None,
        }
    }

    pub fn open(&mut self) {
        self.closed = None;
    }

    pub fn complete(&mut self) {
        self.closed = Some(Utc::now().naive_utc());
    }

    pub fn is_complete(&mut self) -> bool {
        self.closed.is_some()
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash)]
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
