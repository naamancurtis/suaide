#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;

use crate::state::State;
use app::{build_app, handle_matches};
use domain::SuaideError;

mod schema;

mod app;
mod common;
mod database;
mod domain;
mod settings;
mod state;
mod subcommands;

embed_migrations!();

fn main() -> Result<(), SuaideError> {
    let app = build_app();
    let state = State::new()?;
    handle_matches(app.get_matches(), &state)
}
