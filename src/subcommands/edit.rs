use clap::{App, Arg, ArgMatches};

use diesel::prelude::*;
use std::io;

use crate::common::{
    inputs::{get_input, get_optional_input, get_state_input},
    storage::get_task,
};
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

pub fn handler<W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<W>,
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

fn grab_input_from_user<W: io::Write>(
    task: &Task,
    state: &mut State<W>,
) -> Result<TaskChangeSet, SuaideError> {
    let mut change_set = TaskChangeSet::default();
    let description = get_input("description", Some(task.description.clone()))?;
    let ticket = state.generate_ticket_id(get_optional_input("ID", task.ticket.clone())?);
    let status = get_state_input(task.status.into());

    change_set.set_description(task, description);
    change_set.set_ticket(task, ticket);
    change_set.set_status(task, status);
    Ok(change_set)
}
