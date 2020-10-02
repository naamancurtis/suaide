#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::state::State;
use app::{build_app, handle_matches};
use domain::SuaideError;
use std::io;

mod schema;

mod app;
mod common;
mod database;
mod domain;
mod settings;
mod state;
mod subcommands;

fn main() -> Result<(), SuaideError> {
    let app = build_app();
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut state = State::new(&mut writer)?;

    handle_matches(app, &mut state)
}
