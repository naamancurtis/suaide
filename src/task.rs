use crate::enums::Status;
use crate::schema::suaide;
use chrono::prelude::*;
// use chrono::{DateTime, Datelike, Local, NaiveDateTime};
use colored::Colorize;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct Task {
    id: i32,
    ticket: Option<String>,
    description: String,
    opened: i64,
    closed: Option<i64>,
}

impl Task {
    pub fn new(description: String, ticket: Option<String>) -> Self {
        Self {
            id: 0,
            ticket,
            description,
            opened: Local::now().timestamp(),
            closed: None,
        }
    }

    pub fn open(&mut self) {
        self.closed = None;
    }

    pub fn complete(&mut self) {
        self.closed = Some(Local::now().timestamp())
    }

    pub fn toggle(&mut self) {
        if self.closed.is_some() {
            self.closed = None;
            return;
        }
        self.complete();
    }

    pub fn status(&self) -> Status {
        if self.is_complete() {
            return Status::Closed;
        }
        if self.already_in_progress() {
            return Status::InProgress;
        }
        Status::Open
    }

    pub fn list(&self) {
        let ticket = match &self.ticket {
            Some(ticket) => format!("{}:", ticket),
            None => format!("{{#{}:}}", self.id.to_string().italic()),
        };
        println!("[{}] {} {}", self.status(), ticket, self.description);
    }
}

// Private API
impl Task {
    fn is_complete(&self) -> bool {
        self.closed.is_some()
    }

    fn already_in_progress(&self) -> bool {
        let now = Local::now();
        let opened: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(self.opened, 0), Utc);
        let opened: DateTime<Local> = DateTime::from(opened);
        if opened.num_days_from_ce() == now.num_days_from_ce() {
            return false;
        }
        true
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.status().cmp(&other.status()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.opened.cmp(&other.opened) {
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
                Ordering::Less => Ordering::Greater,
            },
            Ordering::Less => Ordering::Less,
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.status() {
            Status::Open => "Start work on",
            Status::InProgress => "Continue with",
            Status::Closed => "Completed",
        };
        if let Some(ticket) = self.ticket.clone() {
            return write!(f, "{} [{}] {}", prefix, ticket, self.description);
        }
        write!(f, "{} {}", prefix, self.description)
    }
}
