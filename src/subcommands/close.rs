use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;

use diesel::prelude::*;

use crate::domain::{Status, SuaideError};

pub fn app() -> App<'static> {
    App::new("close").about("Mark a task as closed").arg(
        Arg::with_name("task")
            .index(1)
            .about("Mark this task as closed")
            .takes_value(true),
    )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    if let Some(task) = matches.value_of("task") {
        return update_task(task, &db_conn);
    }
    Err(SuaideError::IncorrectArgs)
}

fn update_task(task: &str, db_conn: &SqliteConnection) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::*;
    let update = (
        closed.eq(Some(Local::now().timestamp())),
        status.eq(Status::Closed as i16),
    );

    if let Ok(result) = diesel::update(suaide.filter(ticket.eq(Some(task))))
        .set(update)
        .execute(db_conn)
    {
        if result == 1 {
            println!("[{}]: {}", "Completed".yellow(), task);
            return Ok(());
        }
    }

    if let Ok(num) = task.parse::<i32>() {
        if let Ok(result) = diesel::update(suaide.find(num))
            .set(update)
            .execute(db_conn)
        {
            if result == 1 {
                println!("[{}]: {}", "Completed".yellow(), task);
                return Ok(());
            }
        }
    }
    Err(SuaideError::NotFound)
}
