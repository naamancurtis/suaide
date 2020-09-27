use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use dialoguer::Confirm;
use std::io;

use diesel::prelude::*;

use crate::domain::SuaideError;
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("remove")
        .about("Delete task(s)")
        .arg(
            Arg::with_name("task")
                .index(1)
                .about("What ticket would you like to delete")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all")
                .about("Delete all tasks")
                .long("all")
                .short('a')
                .exclusive(true)
                .takes_value(false),
        )
}

pub fn handler<R: io::BufRead, W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    if matches.is_present("all") {
        return confirm_and_delete_all(state.get_conn());
    }

    if let Some(task) = matches.value_of("task") {
        let task = state.generate_ticket_id(Some(task)).unwrap();
        return delete_single_task(&task, state.get_conn());
    }
    Err(SuaideError::IncorrectArgs)
}

fn confirm_and_delete_all(db_conn: &SqliteConnection) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::suaide;

    let mut confirmation = Confirm::new();
    confirmation.default(false);
    if confirmation
        .with_prompt(format!(
            "{} {}",
            "Are you sure?".bold(),
            "This is irreversible".red().italic()
        ))
        .interact()?
    {
        diesel::delete(suaide).execute(db_conn)?;
        println!("{}", "Removed all tasks".red());
    }
    Ok(())
}

fn delete_single_task(task: &str, db_conn: &SqliteConnection) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::{suaide, ticket};

    if let Ok(result) = diesel::delete(suaide.filter(ticket.eq(Some(task)))).execute(db_conn) {
        if result == 1 {
            println!("[{}]: Task {}", "Removed".red(), task);
            return Ok(());
        }
    }
    if let Ok(num) = task.parse::<i32>() {
        if let Ok(result) = diesel::delete(suaide.find(num)).execute(db_conn) {
            if result == 1 {
                println!("[{}]: Task #{}", "Removed".red(), task);
                return Ok(());
            }
        }
    }
    Err(SuaideError::NotFound)
}
