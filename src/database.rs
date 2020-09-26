use diesel::prelude::*;

use crate::domain::SuaideError;

pub fn establish_connection(db_url: &str) -> Result<SqliteConnection, SuaideError> {
    let conn = if cfg!(test) {
        SqliteConnection::establish(":memory:")
            .unwrap_or_else(|_| panic!("Error creating test database"))
    } else {
        SqliteConnection::establish(db_url)?
    };

    diesel_migrations::run_pending_migrations(&conn)?;
    Ok(conn)
}
