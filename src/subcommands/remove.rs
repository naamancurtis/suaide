use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io::stdin;

use diesel::prelude::*;

use crate::domain::SuaideError;

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

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    if matches.is_present("all") {
        return confirm_and_delete_all(&db_conn);
    }

    if let Some(task) = matches.value_of("task") {
        return delete_single_task(task, &db_conn);
    }
    Err(SuaideError::IncorrectArgs)
}

fn confirm_and_delete_all(db_conn: &SqliteConnection) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::*;
    let mut confirmation = String::new();
    println!(
        "{} {} {}",
        "Are you sure?".bold().red(),
        "(y/N)".red(),
        "This process is irreversible".italic().red(),
    );
    stdin().read_line(&mut confirmation).unwrap();
    if confirmation.to_lowercase().remove(0) == 'y' {
        diesel::delete(suaide).execute(db_conn)?;
        println!("Removed all tasks");
    }
    Ok(())
}

fn delete_single_task(task: &str, db_conn: &SqliteConnection) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::*;

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
