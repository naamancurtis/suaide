use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io;

use diesel::prelude::*;

use crate::domain::{Status, SuaideError};
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("close").about("Mark a task as closed").arg(
        Arg::with_name("task")
            .index(1)
            .about("The task to mark as closed")
            .required(true)
            .takes_value(true),
    )
}

pub fn handler<R: io::BufRead, W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    if let Some(task) = matches.value_of("task") {
        return update_task(task, state);
    }
    Err(SuaideError::IncorrectArgs)
}

fn update_task<R: io::BufRead, W: io::Write>(
    task: &str,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::{closed, status, suaide, ticket};

    let update = (
        closed.eq(Some(Local::now().timestamp())),
        status.eq(Status::Closed as i16),
    );

    if let Ok(result) =
        diesel::update(suaide.filter(ticket.eq(state.generate_ticket_id(Some(task)))))
            .set(update)
            .execute(state.get_conn())
    {
        if result == 1 {
            println!("[{}]: {}", "Completed".yellow(), task);
            return Ok(());
        }
    }

    if let Ok(num) = task.parse::<i32>() {
        if let Ok(result) = diesel::update(suaide.find(num))
            .set(update)
            .execute(state.get_conn())
        {
            if result == 1 {
                println!("[{}]: {}", "Completed".yellow(), task);
                return Ok(());
            }
        }
    }
    Err(SuaideError::NotFound)
}
