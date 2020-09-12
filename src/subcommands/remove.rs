use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io::stdin;

use diesel::prelude::*;

use crate::errors::SuaideError;

pub fn app() -> App<'static> {
    App::new("remove")
        .about("Delete a tasks")
        .arg(
            Arg::with_name("ticket")
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
    use crate::schema::suaide::dsl::*;

    if matches.is_present("all") {
        let mut confirmation = String::new();
        println!(
            "{} {}",
            "Are you sure?".bold().red(),
            "(This process is irreversible)".italic().red(),
        );
        println!("{}", "y/N to confirm".italic());
        stdin().read_line(&mut confirmation).unwrap();
        if confirmation.to_lowercase().remove(0) == 'y' {
            diesel::delete(suaide).execute(&db_conn)?;
            println!("Removed all tasks");
        }
        return Ok(());
    }

    if let Some(task) = matches.value_of("ticket") {
        let mut result: usize;
        result = diesel::delete(suaide.filter(ticket.eq(Some(task)))).execute(&db_conn)?;
        if result == 1 {
            println!("[{}]: Task {}", "Removed".truecolor(224, 108, 117), task);
            return Ok(());
        }
        if let Ok(num) = task.parse::<i32>() {
            result = diesel::delete(suaide.filter(ticket.eq(Some(task)).or(id.eq(num))))
                .execute(&db_conn)?;
        }
        if result == 1 {
            println!("[{}]: Task #{}", "Removed".truecolor(224, 108, 117), task);
            return Ok(());
        }
        return Err(SuaideError::NotFound);
    }
    Err(SuaideError::IncorrectArgs)
}
