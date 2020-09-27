use chrono::Local;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io;

use diesel::prelude::*;
use diesel::Insertable;

use crate::domain::SuaideError;
use crate::schema::suaide;
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("add")
        .about("Add new task")
        .arg(
            Arg::with_name("ticket_id")
                .long("ticket")
                .short('t')
                .requires("description")
                .about("Ticket identifier")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("description")
                .long("desc")
                .short('d')
                .about("Description")
                .takes_value(true),
        )
}

pub fn handler<R: io::BufRead, W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    let description: String;
    let ticket: Option<String>;

    if matches.is_present("description") {
        description = matches
            .value_of("description")
            .map(String::from)
            .expect("already checked string");
        ticket = state.generate_ticket_id(matches.value_of("ticket_id").map(String::from));
    } else {
        let result = grab_input_from_user(state)?;
        description = result.0;
        ticket = state.generate_ticket_id(result.1);
    }

    let task = AddTask::new(ticket, description);
    match diesel::insert_into(suaide::table)
        .values(&task)
        .execute(state.get_conn())
    {
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => Err(SuaideError::TicketAlreadyExistsError),
        Err(e) => Err(SuaideError::from(e)),
        Ok(x) => Ok(x),
    }?;
    writeln!(
        state.writer(),
        "{}: {}",
        "Added task".green(),
        task.description
    )?;
    Ok(())
}

fn grab_input_from_user<R: io::BufRead, W: io::Write>(
    state: &mut State<R, W>,
) -> Result<(String, Option<String>), SuaideError> {
    let description = state.get_input("description", None)?;
    let ticket = state.get_optional_input("ID", None)?;
    Ok((description, ticket))
}

#[derive(Insertable)]
#[table_name = "suaide"]
struct AddTask {
    ticket: Option<String>,
    description: String,
    opened: i64,
    status: i16,
}

impl AddTask {
    pub fn new(ticket: Option<String>, description: String) -> Self {
        Self {
            ticket,
            description,
            opened: Local::now().timestamp(),
            status: 0,
        }
    }
}

#[cfg(test)]
mod test_add_app {
    use super::*;

    use crate::domain::{Status, Task};
    use crate::schema::suaide::dsl::*;
    use crate::state::State;
    use std::io::Cursor;
    use std::str::from_utf8;

    const EXPECTED_STDOUT_OUTPUT: &str = "\u{1b}[32mAdded task\u{1b}[0m: Test Description\n";

    #[test]
    fn test_full_flag_inputs_short() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();
        let matches = app().get_matches_from(vec!["add", "-t", "1234", "-d", "Test Description"]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, Some("1234".to_string()));
        assert_eq!(result.description, "Test Description".to_string());
        assert_eq!(result.status, Status::Open as i16);
        assert_eq!(result.closed, None);

        let data = from_utf8(&writer).expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
    }

    #[test]
    fn test_full_flag_inputs_short_no_ticket() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();
        let matches = app().get_matches_from(vec!["add", "-d", "Test Description"]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, None);
        assert_eq!(result.description, "Test Description".to_string());
        assert_eq!(result.status, Status::Open as i16);
        assert_eq!(result.closed, None);

        let data = from_utf8(&writer).expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
    }

    #[test]
    fn test_full_flag_inputs_long() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();
        let matches = app().get_matches_from(vec![
            "add",
            "--ticket",
            "1234",
            "--desc",
            "Test Description",
        ]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, Some("1234".to_string()));
        assert_eq!(result.description, "Test Description".to_string());
        assert_eq!(result.status, Status::Open as i16);
        assert_eq!(result.closed, None);

        let data = from_utf8(&writer).expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
    }

    #[test]
    fn test_full_flag_inputs_long_no_ticket() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();
        let matches = app().get_matches_from(vec!["add", "--desc", "Test Description"]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, None);
        assert_eq!(result.description, "Test Description".to_string());
        assert_eq!(result.status, Status::Open as i16);
        assert_eq!(result.closed, None);

        let data = from_utf8(&writer).expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
    }

    #[test]
    fn test_full_flag_inputs_short_errors_with_no_description() {
        let matches = app().try_get_matches_from(vec!["add", "-t", "1234"]);
        assert!(matches.is_err());
    }

    #[test]
    fn test_full_flag_inputs_long_errors_with_no_description() {
        let matches = app().try_get_matches_from(vec!["add", "-ticket", "1234"]);
        assert!(matches.is_err());
    }

    #[test]
    fn test_ticket_id_already_exists() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();

        test_helpers::insert_task(state.get_conn());

        let matches = app().get_matches_from(vec!["add", "-t", "1234", "-d", "Test Description"]);
        let result = handler(&matches, &mut state).unwrap_err();
        match result {
            SuaideError::TicketAlreadyExistsError => {}
            _ => panic!("Expected ticket already exists error"),
        }
    }

    #[test]
    fn test_prompts() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"Test Description\n1234\n");
        let mut state = State::new(&mut reader, &mut writer).unwrap();
        let matches = app().get_matches_from(vec!["add"]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, Some("1234".to_string()));
        assert_eq!(result.description, "Test Description".to_string());
        assert_eq!(result.status, Status::Open as i16);
        assert_eq!(result.closed, None);

        let data = from_utf8(&writer[writer.len() - EXPECTED_STDOUT_OUTPUT.len()..])
            .expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
    }

    #[test]
    fn test_prompts_error_on_duplicate_id() {
        let mut writer: Vec<u8> = Vec::new();
        let mut reader = Cursor::new(b"Test Description\n1234\n");
        let mut state = State::new(&mut reader, &mut writer).unwrap();

        test_helpers::insert_task(state.get_conn());

        let matches = app().get_matches_from(vec!["add"]);
        let result = handler(&matches, &mut state).unwrap_err();
        match result {
            SuaideError::TicketAlreadyExistsError => {}
            _ => panic!("Expected ticket already exists error"),
        }
    }
}

#[cfg(test)]

pub(crate) mod test_helpers {
    use super::AddTask;
    use diesel::prelude::*;

    pub(crate) fn insert_task(db_conn: &SqliteConnection) {
        let task = AddTask {
            ticket: Some("1234".to_string()),
            description: "Test Description".to_string(),
            status: 0,
            opened: 10000,
        };

        diesel::insert_into(crate::schema::suaide::table)
            .values(task)
            .execute(db_conn)
            .expect("Insert should be successful");
    }
}
