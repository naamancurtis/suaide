#[macro_use]
extern crate diesel;

use app::{build_app, handle_matches};
use diesel::prelude::*;
use domain::SuaideError;
use std::env;

pub mod schema;

mod app;
mod common;
mod domain;
mod subcommands;

fn main() -> Result<(), SuaideError> {
    let app = build_app();

    let db_url = env::var("SUAIDE_DB_URL").unwrap_or_else(|_| "suaide.sqlite".to_string());
    let conn = SqliteConnection::establish(&db_url)?;

    handle_matches(app.get_matches(), conn)
}
