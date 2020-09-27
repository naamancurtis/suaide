use colored::Colorize;
use diesel::SqliteConnection;
use std::io;

use crate::common::{ADD_PREFIX, EDIT_PREFIX};
use crate::database::establish_connection;
use crate::domain::SuaideError;
use crate::settings::Settings;

pub struct State<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    settings: Settings,
    conn: SqliteConnection,
    w: W,
    r: R,
}

impl<R, W> State<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(reader: R, writer: W) -> Result<Self, SuaideError> {
        let settings = Settings::new()?;
        let conn = establish_connection(&settings.db_url)?;
        Ok(State {
            settings,
            conn,
            w: writer,
            r: reader,
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

    pub fn writer(&mut self) -> &mut W {
        &mut self.w
    }

    pub fn reader(&mut self) -> &mut R {
        &mut self.r
    }

    pub fn get_input(
        &mut self,
        key: &str,
        existing_data: Option<String>,
    ) -> Result<String, SuaideError> {
        let prefix = if existing_data.is_some() {
            EDIT_PREFIX
        } else {
            ADD_PREFIX
        };

        let mut input = existing_data.unwrap_or_default();

        writeln!(self.writer(), "{}", format!("{} task {}", prefix, key))?;

        while input.is_empty() {
            self.reader().read_line(&mut input)?;

            let len = input.trim_end_matches(&['\r', '\n'][..]).len();
            input.truncate(len);
            if input.is_empty() {
                writeln!(self.writer(), "{} is required", key.yellow().italic())?;
            }
        }

        Ok(input)
    }

    pub fn get_optional_input(
        &mut self,
        key: &str,
        existing_data: Option<String>,
    ) -> Result<Option<String>, SuaideError> {
        let prefix = if existing_data.is_some() {
            EDIT_PREFIX
        } else {
            ADD_PREFIX
        };

        let mut input = existing_data.unwrap_or_default();

        writeln!(
            self.writer(),
            "{}",
            format!("{} task {} {}", prefix, key, "(Enter to skip)".italic())
        )?;

        self.reader().read_line(&mut input)?;

        let len = input.trim_end_matches(&['\r', '\n'][..]).len();
        input.truncate(len);

        if input.is_empty() {
            return Ok(None);
        }

        Ok(Some(input))
    }
}
