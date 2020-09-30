use clap::{App, Arg, ArgMatches};
use std::io;

use diesel::prelude::*;

use crate::common::{inputs::get_state_input, storage::get_task};
use crate::domain::{Status, SuaideError, Task, TaskChangeSet};
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("status")
        .about("Change the status of a task")
        .arg(
            Arg::with_name("task")
                .index(1)
                .about("The task to update")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("state")
                .index(2)
                .about("The state to update the task with")
                .possible_values(&[
                    "open",
                    "o",
                    "in-progress",
                    "inprogress",
                    "progress",
                    "ip",
                    "closed",
                    "close",
                    "c",
                    "cancel",
                    "cancelled",
                ])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .about("Provide additional information about each task"),
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
        let change_set = if let Some(state) = matches.value_of("state") {
            let updated_status = Status::from(state);
            generate_change_set(&task, updated_status)?
        } else {
            let updated_status = get_state_input(task.status.into());
            generate_change_set(&task, updated_status)?
        };

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

fn generate_change_set(task: &Task, status: Status) -> Result<TaskChangeSet, SuaideError> {
    let mut change_set = TaskChangeSet::default();
    change_set.set_status(task, status);
    Ok(change_set)
}
