#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;

use app::{build_app, handle_matches};
use database::establish_connection;
use domain::SuaideError;

mod schema;

mod app;
mod common;
mod database;
mod domain;
mod subcommands;

embed_migrations!();

fn main() -> Result<(), SuaideError> {
    let app = build_app();
    let conn = establish_connection()?;
    handle_matches(app.get_matches(), conn)
}
