use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io;

use diesel::prelude::*;

use crate::domain::{Status, SuaideError};
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("close").about("Mark a task as closed").arg(
        Arg::with_name("task")
            .index(1)
            .about("The task to mark as closed")
            .required(true)
            .takes_value(true),
    )
}

pub fn handler<R: io::BufRead, W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    if let Some(task) = matches.value_of("task") {
        return update_task(task, state);
    }
    Err(SuaideError::IncorrectArgs)
}

fn update_task<R: io::BufRead, W: io::Write>(
    task: &str,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    use crate::schema::suaide::dsl::{closed, status, suaide, ticket};

    let update = (
        closed.eq(Some(Local::now().timestamp())),
        status.eq(Status::Closed as i16),
    );

    if let Ok(result) =
        diesel::update(suaide.filter(ticket.eq(state.generate_ticket_id(Some(task)))))
            .set(update)
            .execute(state.get_conn())
    {
        if result == 1 {
            writeln!(state.writer(), "[{}]: {}", "Completed".yellow(), task)?;
            return Ok(());
        }
    }

    if let Ok(num) = task.parse::<i32>() {
        if let Ok(result) = diesel::update(suaide.find(num))
            .set(update)
            .execute(state.get_conn())
        {
            if result == 1 {
                writeln!(state.writer(), "[{}]: {}", "Completed".yellow(), task)?;
                return Ok(());
            }
        }
    }
    Err(SuaideError::NotFound)
}

#[cfg(test)]
mod test_close_app {
    use super::*;

    use crate::domain::{Status, Task};
    use crate::schema::suaide::dsl::*;
    use crate::state::{new_test_io_state, State};

    use std::str::from_utf8;

    const EXPECTED_STDOUT_OUTPUT: &str = "[\u{1b}[33mCompleted\u{1b}[0m]: 1234\n";

    #[test]
    fn happy_path() {
        let (mut reader, mut writer) = new_test_io_state(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();

        test_helpers::insert_task(state.get_conn());

        let matches = app().get_matches_from(vec!["close", "1234"]);
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
        assert_eq!(result.status, Status::Closed as i16);
        assert!(result.closed.is_some());

        let data = from_utf8(&writer).expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
    }

    #[test]
    fn should_error_with_not_found() {
        let (mut reader, mut writer) = new_test_io_state(b"");
        let mut state = State::new(&mut reader, &mut writer).unwrap();
        let matches = app().get_matches_from(vec!["close", "1234"]);
        let result = handler(&matches, &mut state).unwrap_err();
        match result {
            SuaideError::NotFound => {}
            _ => panic!("Expected Not Found error"),
        };
    }
}

#[cfg(test)]
mod test_helpers {
    use crate::domain::AddTask;
    use diesel::prelude::*;

    pub fn insert_task(db_conn: &SqliteConnection) {
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
