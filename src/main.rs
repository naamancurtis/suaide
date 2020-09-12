#[macro_use]
extern crate diesel;

use clap::App;
use diesel::prelude::*;
use errors::SuaideError;
use std::env;
use subcommands::*;

mod common;
mod enums;
mod errors;
pub mod schema;
mod subcommands;
mod task;

fn main() -> Result<(), SuaideError> {
    let matches = App::new("suaide")
        .version("0.1")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .about("A simple cli app to track tasks and auto-generate stand-up reports")
        .subcommand(add::app())
        .subcommand(edit::app())
        .subcommand(list::app())
        .subcommand(remove::app())
        .subcommand(close::app())
        .subcommand(App::new("toggle").about("Toggle the state of a task"))
        .subcommand(App::new("stand-up").about("Output stand-up report"))
        .get_matches();

    let db_url = env::var("SUAIDE_DB_URL").unwrap_or_else(|_| "suaide.sqlite".to_string());
    let conn = SqliteConnection::establish(&db_url)?;

    match matches.subcommand() {
        ("add", Some(matches)) => add::handler(matches, conn),
        ("list", Some(matches)) => list::handler(matches, conn),
        ("remove", Some(matches)) => remove::handler(matches, conn),
        ("close", Some(matches)) => close::handler(matches, conn),
        ("edit", Some(matches)) => edit::handler(matches, conn),
        _ => Err(SuaideError::SubCommandNotFound),
    }
}
