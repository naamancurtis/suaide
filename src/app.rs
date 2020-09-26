use clap::{App, ArgMatches};

use crate::domain::SuaideError;
use crate::state::State;
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

pub(crate) fn handle_matches(matches: ArgMatches, state: &State) -> Result<(), SuaideError> {
    match matches.subcommand() {
        ("add", Some(matches)) => add::handler(matches, state),
        ("list", Some(matches)) => list::handler(matches, state),
        ("remove", Some(matches)) => remove::handler(matches, state),
        ("close", Some(matches)) => close::handler(matches, state),
        ("edit", Some(matches)) => edit::handler(matches, state),
        ("status", Some(matches)) => status::handler(matches, state),
        ("standup", Some(matches)) => stand_up::handler(matches, state),
        _ => Err(SuaideError::SubCommandNotFound),
    }
}
