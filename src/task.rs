use crate::enums::Status;
use crate::schema::suaide;
use chrono::prelude::*;
// use chrono::{DateTime, Datelike, Local, NaiveDateTime};
use colored::Colorize;
use diesel::{AsChangeset, Queryable};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct Task {
    pub(crate) id: i32,
    pub(crate) ticket: Option<String>,
    pub(crate) description: String,
    pub(crate) status: i16,
    pub(crate) opened: i64,
    pub(crate) closed: Option<i64>,
}

#[derive(AsChangeset, Default)]
#[table_name = "suaide"]
pub(crate) struct TaskChangeSet {
    ticket: Option<Option<String>>,
    description: Option<String>,
    status: Option<i16>,
    opened: Option<i64>,
    closed: Option<Option<i64>>,
}

impl Task {
    pub fn new(description: String, ticket: Option<String>) -> Self {
        Self {
            id: 0,
            ticket,
            description,
            status: 0,
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

    pub fn task_status(&self) -> Status {
        self.status.into()
    }

    pub fn print(&self) {
        let ticket = match &self.ticket {
            Some(ticket) => format!("{}:", ticket),
            None => format!("#{}:", self.id.to_string().italic()),
        };
        println!("[{}] {} {}", self.task_status(), ticket, self.description);
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

impl TaskChangeSet {
    pub(crate) fn set_description(&mut self, task: &Task, description: String) {
        if task.description != description {
            self.description = Some(description);
        }
    }

    pub(crate) fn set_ticket(&mut self, task: &Task, ticket: Option<String>) {
        if task.ticket != ticket {
            self.ticket = Some(ticket);
        }
    }

    pub(crate) fn set_status(&mut self, task: &Task, status: Status) {
        if task.status != status as i16 {
            self.status = Some(status as i16);
        }
    }

    pub(crate) fn set_closed(&mut self, task: &Task, closed: Option<i64>) {
        if task.closed != closed {
            self.closed = Some(closed);
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.task_status().cmp(&other.task_status()) {
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
        let prefix = match self.task_status() {
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
