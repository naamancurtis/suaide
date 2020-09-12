use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};

use diesel::prelude::*;

use crate::errors::SuaideError;

pub fn app() -> App<'static> {
    App::new("done").about("Mark a task as done").arg(
        Arg::with_name("task")
            .index(1)
            .about("Mark this task as done")
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

    if let Ok(result) = diesel::update(suaide.filter(ticket.eq(Some(task))))
        .set(closed.eq(Some(Local::now().timestamp())))
        .execute(db_conn)
    {
        if result == 1 {
            return Ok(());
        }
    }

    if let Ok(num) = task.parse::<i32>() {
        if let Ok(result) = diesel::update(suaide.find(num))
            .set(closed.eq(Some(Local::now().timestamp())))
            .execute(db_conn)
        {
            if result == 1 {
                return Ok(());
            }
        }
    }
    Err(SuaideError::NotFound)
}
