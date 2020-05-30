use crate::enums::Priority;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}, Task: {}", self.id, self.message)
    }
}
