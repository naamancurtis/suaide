use clap::{App, Arg, ArgMatches};

use diesel::prelude::*;
use std::io;

use crate::common::{inputs::get_state_input, storage::get_task};
use crate::domain::{SuaideError, Task, TaskChangeSet};
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("edit")
        .about("Edit a task")
        .arg(
            Arg::with_name("task")
                .index(1)
                .required(true)
                .about("The task to edit")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .about("Provide additional information about each task"),
        )
}

pub fn handler<R: io::BufRead, W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    let is_verbose = matches.is_present("verbose");
    if let Some(task_id) = matches.value_of("task") {
        let task_id = state.generate_ticket_id(Some(task_id)).unwrap();
        let task = get_task(&task_id, state.get_conn())?;
        let change_set = grab_input_from_user(&task, state)?;

        use crate::schema::suaide::dsl::suaide;

        diesel::update(suaide.find(task.id))
            .set(change_set)
            .execute(state.get_conn())?;

        let task = get_task(&task_id, state.get_conn())?;
        task.print(is_verbose);
        return Ok(());
    }
    Err(SuaideError::IncorrectArgs)
}

fn grab_input_from_user<R: io::BufRead, W: io::Write>(
    task: &Task,
    state: &mut State<R, W>,
) -> Result<TaskChangeSet, SuaideError> {
    let mut change_set = TaskChangeSet::default();
    let description = state.get_input("description", Some(task.description.clone()))?;
    let ticket_id = state.get_optional_input("ID", task.ticket.clone())?;
    let ticket = state.generate_ticket_id(ticket_id);
    let status = get_state_input(task.status.into());

    change_set.set_description(task, description);
    change_set.set_ticket(task, ticket);
    change_set.set_status(task, status);
    Ok(change_set)
}

#[cfg(test)]
mod test_edit_app {
    use super::*;

    use crate::domain::{Status, Task};
    use crate::schema::suaide::dsl::*;
    use crate::state::{new_test_io_state, State};

    use std::str::from_utf8;

    const EXPECTED_STDOUT_OUTPUT: &str = "\u{1b}[32mAdded task\u{1b}[0m: Test Description\n";

    #[ignore]
    #[test]
    fn should_edit_a_task() {
        let (mut reader, mut writer) = new_test_io_state(b"Super Description\n4321\n");
        let mut state = State::new(&mut reader, &mut writer).unwrap();

        test_helpers::insert_task(state.get_conn());

        let matches = app().get_matches_from(vec!["task", "1234"]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, Some("4321".to_string()));
        assert_eq!(result.description, "Super Description".to_string());
        assert_eq!(result.status, Status::Open as i16);
        assert_eq!(result.closed, None);

        let data = from_utf8(&writer).expect("should be a string here");
        assert_eq!(data, EXPECTED_STDOUT_OUTPUT);
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
            status: 3,
            opened: 10000,
        };

        diesel::insert_into(crate::schema::suaide::table)
            .values(task)
            .execute(db_conn)
            .expect("Insert should be successful");
    }
}
