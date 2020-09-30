use colored::Colorize;
use dialoguer::Input;
use diesel::SqliteConnection;
use std::io;

use crate::common::{ADD_PREFIX, EDIT_PREFIX};
use crate::database::establish_connection;
use crate::domain::SuaideError;
use crate::settings::Settings;

pub struct State<W>
where
    W: io::Write,
{
    settings: Settings,
    conn: SqliteConnection,
    w: W,
}

impl<W> State<W>
where
    W: io::Write,
{
    pub fn new(writer: W) -> Result<Self, SuaideError> {
        let mut settings = Settings::new()?;

        // Easier to set this to "" within tests, or it might pick
        // up on a config file and run the tests with unexpected
        // behavior
        if cfg!(test) {
            settings.ticket_prefix = "".to_string();
        }

        let conn = establish_connection(&settings.db_url)?;
        Ok(State {
            settings,
            conn,
            w: writer,
        })
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

    pub fn set_prefix(&mut self, prefix: String) {
        self.settings.ticket_prefix = prefix;
    }

    pub fn writer(&mut self) -> &mut W {
        &mut self.w
    }

    pub fn get_input(
        &mut self,
        key: &str,
        existing_data: Option<String>,
    ) -> Result<String, SuaideError> {
        // @todo - find a way to properly test Dialoguer
        if cfg!(test) {
            return Ok("MOCK DATA".to_string());
        }

        let prefix = if existing_data.is_some() {
            EDIT_PREFIX
        } else {
            ADD_PREFIX
        };

        let mut input = Input::<String>::new();
        let input = input
            .with_prompt(format!("{} task {}", prefix, key))
            .allow_empty(false)
            .with_initial_text(existing_data.unwrap_or_default());
        Ok(input.interact()?)
    }

    pub fn get_optional_input(
        &mut self,
        key: &str,
        existing_data: Option<String>,
    ) -> Result<Option<String>, SuaideError> {
        // @todo - find a way to properly test Dialoguer
        if cfg!(test) {
            return Ok(Some("MOCK DATA".to_string()));
        }

        let (prefix, suffix) = if existing_data.is_some() {
            (EDIT_PREFIX, "".italic())
        } else {
            (ADD_PREFIX, "(Enter to skip)".italic())
        };

        let mut input = Input::<String>::new();
        let input = input
            .with_prompt(format!("{} task {} {}", prefix, key, suffix))
            .allow_empty(true)
            .with_initial_text(existing_data.unwrap_or_default());
        let final_input = input.interact()?;

        if final_input.is_empty() {
            return Ok(None);
        }

        Ok(Some(final_input))
    }
}

#[cfg(test)]
mod test_state_methods {
    use super::*;

    // These tests are kind of useless?
    // Look into how to improve them

    #[test]
    fn get_input_without_data() {
        let mut writer = Vec::new();
        let mut state = State::new(&mut writer).unwrap();
        let output = state.get_input("TEST", None);
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), "MOCK DATA".to_string());
    }

    #[test]
    fn get_input_with_data() {
        let mut writer = Vec::new();
        let mut state = State::new(&mut writer).unwrap();
        let output = state.get_input("TEST", Some("EXISTING TEXT".to_string()));
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), "MOCK DATA".to_string());
    }
}
