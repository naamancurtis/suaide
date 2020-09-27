use clap::{App, Arg, ArgMatches};
use std::io;

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
        .arg(
            Arg::with_name("prefix")
                .about("Overwrite the ticket prefix")
                .takes_value(true)
                .short('p')
                .long("prefix"),
        )
}

pub(crate) fn handle_matches<R, W>(
    matches: ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError>
where
    W: io::Write,
    R: io::BufRead,
{
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

#[cfg(test)]
mod test_build_app {
    use super::build_app;

    #[test]
    fn add_is_present() {
        let app = build_app();
        let subcommands = app
            .get_subcommands()
            .iter()
            .map(|app| app.get_name())
            .collect::<Vec<&str>>();
        let expected = [
            "add", "edit", "list", "remove", "close", "status", "standup",
        ];
        assert_eq!(subcommands, expected);
    }
}
