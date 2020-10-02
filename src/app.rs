use clap::{App, AppSettings, Arg};
use std::io;

use crate::domain::SuaideError;
use crate::state::State;
use crate::subcommands::*;

pub fn build_app<'a>() -> App<'a, 'static> {
    App::new("Suaide")
        .version("v0.1")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .help("A simple cli app to track tasks and auto-generate stand-up reports")
        .global_settings(&[AppSettings::ColorAuto, AppSettings::VersionlessSubcommands])
        .subcommand(add::app())
        .subcommand(edit::app())
        .subcommand(list::app())
        .subcommand(remove::app())
        .subcommand(close::app())
        .subcommand(status::app())
        .subcommand(stand_up::app())
        .arg(
            Arg::with_name("prefix")
                .help("Overwrite the ticket prefix")
                .takes_value(true)
                .short("p")
                .long("prefix"),
        )
}

pub(crate) fn handle_matches<'a, W>(
    app: App<'a, 'static>,
    state: &mut State<W>,
) -> Result<(), SuaideError>
where
    W: io::Write,
{
    let matches = app.get_matches();

    if let Some(prefix) = matches.value_of("prefix") {
        state.set_prefix(prefix.to_string());
    }

    match matches.subcommand() {
        ("add", Some(matches)) => add::handler(matches, state),
        ("edit", Some(matches)) => edit::handler(matches, state),
        ("list", Some(matches)) => list::handler(matches, state),
        ("remove", Some(matches)) => remove::handler(matches, state),
        ("close", Some(matches)) => close::handler(matches, state),
        ("status", Some(matches)) => status::handler(matches, state),
        ("standup", Some(matches)) => stand_up::handler(matches, state),
        _ => Err(SuaideError::SubCommandNotFound),
    }
}
