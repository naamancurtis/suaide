#[macro_use]
extern crate diesel;

use crate::enums::Priority;
use clap::{App, Arg};
use diesel::prelude::*;
use errors::SuaideError;
use schema::*;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use subcommands::*;

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
        .subcommand(App::new("edit").about("Edit an existing task"))
        .subcommand(App::new("list").about("List tasks"))
        .subcommand(App::new("remove").about("Delete a task"))
        .subcommand(App::new("done").about("Mark a task as done"))
        .subcommand(App::new("toggle").about("Toggle the state of a task"))
        .subcommand(App::new("stand-up").about("Output stand-up report"))
        .get_matches();

    let db_url = env::var("SUAIDE_DB_URL").unwrap_or_else(|_| "suaide.sqlite".to_string());
    let conn = SqliteConnection::establish(&db_url)?;

    match matches.subcommand() {
        ("add", Some(matches)) => add::add(matches, conn),
        _ => Err(SuaideError::SubCommandNotFound),
    }
}
