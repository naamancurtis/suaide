use chrono::prelude::*;
use colored::Colorize;
use diesel::{AsChangeset, Queryable};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::time::{Duration, UNIX_EPOCH};

use crate::common::DATE_FORMAT;
use crate::domain::Status;
use crate::schema::suaide;

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
    pub fn task_status(&self) -> Status {
        self.status.into()
    }

    pub fn print(&self, verbose: bool) {
        if verbose {
            let ticket = match &self.ticket {
                Some(ticket) => format!("{}:", ticket),
                None => format!("#{}:", self.id.to_string().italic()),
            };
            println!("[{}] {} {}", self.task_status(), ticket, self.description);
            println!(
                "\t{:30} {}",
                format!("Opened: {}", self.opened_to_string()),
                format!("Closed: {}", self.closed_to_string())
            );
            println!();
            return;
        }

        let ticket = match &self.ticket {
            Some(ticket) => format!("{}:", ticket),
            None => format!("#{}:", self.id.to_string().italic()),
        };
        println!("[{}] {} {}", self.task_status(), ticket, self.description);
    }
}

// Private API
impl Task {
    fn opened_to_string(&self) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(self.opened as u64);
        let date = DateTime::<Local>::from(d);
        date.format(DATE_FORMAT).to_string()
    }

    fn closed_to_string(&self) -> String {
        if let Some(closed) = self.closed {
            let d = UNIX_EPOCH + Duration::from_secs(closed as u64);
            let date = DateTime::<Local>::from(d);
            date.format(DATE_FORMAT).to_string()
        } else {
            "".to_string()
        }
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
            match status {
                Status::Closed | Status::Cancelled => {
                    self.set_closed(task, Some(Local::now().timestamp()))
                }
                _ => self.set_closed(task, None),
            };
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
