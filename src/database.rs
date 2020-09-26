use diesel::prelude::*;

use lazy_static::lazy_static;
use std::env;
use std::fs;
use std::path::Path;

use crate::domain::SuaideError;

lazy_static! {
    static ref DEFAULT_SQLITE_PATH: std::borrow::Cow<'static, str> =
        shellexpand::tilde("~/.suaide.sqlite");
}

pub fn establish_connection() -> Result<SqliteConnection, SuaideError> {
    if cfg!(test) {
        let conn = SqliteConnection::establish(":memory:")
            .unwrap_or_else(|_| panic!("Error creating test database"));

        diesel_migrations::run_pending_migrations(&conn)?;
        return Ok(conn);
    }
    if !Path::new(&DEFAULT_SQLITE_PATH.to_string()).exists() {
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&DEFAULT_SQLITE_PATH.to_string())?;
    };
    let db_url = env::var("SUAIDE_DB_PATH").unwrap_or_else(|_| DEFAULT_SQLITE_PATH.to_string());
    let conn = SqliteConnection::establish(&db_url)?;
    diesel_migrations::run_pending_migrations(&conn)?;

    Ok(conn)
}
