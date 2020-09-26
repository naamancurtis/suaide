use diesel::SqliteConnection;

use crate::database::establish_connection;
use crate::domain::SuaideError;
use crate::settings::Settings;

pub struct State {
    settings: Settings,
    conn: SqliteConnection,
}

impl State {
    pub fn new() -> Result<Self, SuaideError> {
        let settings = Settings::new()?;
        let conn = establish_connection(&settings.db_url)?;
        Ok(State { settings, conn })
    }

    pub fn get_conn(&self) -> &SqliteConnection {
        &self.conn
    }

    pub fn generate_ticket_id(&self, ticket: Option<impl ToString>) -> Option<String> {
        if let Some(t) = ticket {
            let t = t.to_string();
            if t.starts_with(self.get_ticket_prefix()) {
                return Some(t);
            }
            return Some(self.settings.generate_ticket_id(t));
        }
        None
    }

    pub fn get_ticket_prefix(&self) -> &str {
        &self.settings.ticket_prefix
    }
}