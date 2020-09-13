#[macro_use]
extern crate diesel;

use clap::App;
use diesel::prelude::*;
use errors::SuaideError;
use std::env;
use subcommands::*;

pub mod schema;

mod common;
mod enums;
mod errors;
mod subcommands;
mod task;

fn main() -> Result<(), SuaideError> {
    let matches = App::new("suaide")
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
        .get_matches();

    let db_url = env::var("SUAIDE_DB_URL").unwrap_or_else(|_| "suaide.sqlite".to_string());
    let conn = SqliteConnection::establish(&db_url)?;

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
