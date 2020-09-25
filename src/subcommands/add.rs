use chrono::Local;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;

use diesel::prelude::*;
use diesel::Insertable;

use crate::common::inputs::{get_input, get_optional_input};
use crate::domain::SuaideError;
use crate::schema::suaide;

pub fn app() -> App<'static> {
    App::new("add")
        .about("Add new task")
        .arg(
            Arg::with_name("ticket_id")
                .long("ticket")
                .short('t')
                .about("Ticket identifier")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("description")
                .long("desc")
                .short('d')
                .about("Description")
                .takes_value(true),
        )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    let description: String;
    let ticket: Option<String>;

    if matches.is_present("description") {
        description = matches
            .value_of("description")
            .map(String::from)
            .expect("already checked string");
        ticket = matches.value_of("ticket_id").map(String::from);
    } else {
        let result = grab_input_from_user()?;
        description = result.0;
        ticket = result.1;
    }

    let task = AddTask::new(ticket, description);
    let _ = diesel::insert_into(suaide::table)
        .values(&task)
        .execute(&db_conn)?;
    println!("{}: {}", "Added task".green(), task.description);
    Ok(())
}

fn grab_input_from_user() -> Result<(String, Option<String>), SuaideError> {
    let description = get_input("description", None)?;
    let ticket = get_optional_input("ID", None)?;
    Ok((description, ticket))
}

#[derive(Insertable)]
#[table_name = "suaide"]
struct AddTask {
    ticket: Option<String>,
    description: String,
    opened: i64,
    status: i16,
}

impl AddTask {
    pub fn new(ticket: Option<String>, description: String) -> Self {
        Self {
            ticket,
            description,
            opened: Local::now().timestamp(),
            status: 0,
        }
    }
}

#[cfg(test)]
mod app_test {
    use super::app;

    #[test]
    fn builds_app() {
        let app = app();
    }
}
