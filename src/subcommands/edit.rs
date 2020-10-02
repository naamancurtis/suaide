use clap::{App, Arg, ArgMatches};

use diesel::prelude::*;
use std::io;

use crate::common::{inputs::get_state_input, storage::get_task};
use crate::domain::{SuaideError, Task, TaskChangeSet};
use crate::state::State;

pub fn app<'a>() -> App<'a, 'static> {
    App::new("edit")
        .about("Edit a task")
        .arg(
            Arg::with_name("task")
                .index(1)
                .required(true)
                .help("The task to edit")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Provide additional information about each task"),
        )
}

pub fn handler<W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<W>,
) -> Result<(), SuaideError> {
    let is_verbose = matches.is_present("verbose");
    if let Some(task_id) = matches.value_of("task") {
        let task_id = state.generate_ticket_id(Some(task_id)).unwrap();
        let task = get_task(&task_id, state.get_conn())?;
        let change_set = grab_input_from_user(&task, state)?;

        use crate::schema::suaide::dsl::{id, suaide};

        diesel::update(suaide.find(task.id))
            .set(change_set)
            .execute(state.get_conn())?;

        let task = suaide
            .filter(id.eq(task.id))
            .limit(1)
            .load::<Task>(state.get_conn())?
            .pop();

        if let Some(task) = task {
            task.print(is_verbose);
        }
        return Ok(());
    }
    Err(SuaideError::IncorrectArgs)
}

fn grab_input_from_user<W: io::Write>(
    task: &Task,
    state: &mut State<W>,
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
    use crate::state::State;

    #[test]
    fn should_edit_a_task() {
        let mut writer = Vec::new();
        let mut state = State::new(&mut writer).unwrap();

        test_helpers::insert_task(state.get_conn());

        let matches = app().get_matches_from(vec!["edit", "1234"]);
        let result = handler(&matches, &mut state);
        assert!(result.is_ok());

        let db_conn = state.get_conn();
        let result: Task = suaide
            .find(1)
            .first(db_conn)
            .expect("This should return an Ok");

        assert_eq!(result.id, 1);
        assert_eq!(result.ticket, Some("MOCK DATA".to_string()));
        assert_eq!(result.description, "MOCK DATA".to_string());
        assert_eq!(result.status, Status::Cancelled as i16);
        assert_eq!(result.closed, None);
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
