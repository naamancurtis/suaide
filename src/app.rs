use clap::{App, ArgMatches};
use diesel::SqliteConnection;

use crate::domain::SuaideError;
use crate::subcommands::*;

pub fn build_app() -> App<'static> {
    App::new("Suaide")
        .version("v0.1")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .about("A simple cli app to track tasks and auto-generate stand-up reports")
        .subcommand(add::app())
        .subcommand(edit::app())
        .subcommand(list::app())
        .subcommand(remove::app())
        .subcommand(close::app())
        .subcommand(status::app())
        .subcommand(stand_up::app())
}

pub fn handle_matches(matches: ArgMatches, conn: SqliteConnection) -> Result<(), SuaideError> {
    match matches.subcommand() {
        ("add", Some(matches)) => add::handler(matches, conn),
        ("list", Some(matches)) => list::handler(matches, conn),
        ("remove", Some(matches)) => remove::handler(matches, conn),
        ("close", Some(matches)) => close::handler(matches, conn),
        ("edit", Some(matches)) => edit::handler(matches, conn),
        ("status", Some(matches)) => status::handler(matches, conn),
        ("standup", Some(matches)) => stand_up::handler(matches, conn),
        _ => Err(SuaideError::SubCommandNotFound),
    }
}
