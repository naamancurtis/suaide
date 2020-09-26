use clap::{App, Arg, ArgMatches};

use chrono::prelude::*;
use diesel::prelude::*;

use crate::common::{
    inputs::{get_input, get_optional_input, get_state_input},
    storage::get_task,
};
use crate::domain::{Status, SuaideError, Task, TaskChangeSet};

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

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    let is_verbose = matches.is_present("verbose");
    if let Some(task_id) = matches.value_of("task") {
        let task = get_task(task_id, &db_conn)?;
        let change_set = grab_input_from_user(&task)?;

        use crate::schema::suaide::dsl::*;

        diesel::update(suaide.find(task.id))
            .set(change_set)
            .execute(&db_conn)?;

        let task = get_task(task_id, &db_conn)?;
        task.print(is_verbose);
        return Ok(());
    }
    Err(SuaideError::IncorrectArgs)
}

fn grab_input_from_user(task: &Task) -> Result<TaskChangeSet, SuaideError> {
    let mut change_set = TaskChangeSet::default();
    let description = get_input("description", Some(task.description.clone()))?;
    let ticket = get_optional_input("ID", task.ticket.clone())?;
    let status = get_state_input(task.status.into());

    change_set.set_description(task, description);
    change_set.set_ticket(task, ticket);
    change_set.set_status(task, status);
    if task.closed.is_some() && status != Status::Closed {
        change_set.set_closed(task, None);
    }
    if status == Status::Closed || status == Status::Cancelled {
        change_set.set_closed(task, Some(Local::now().timestamp()))
    }
    Ok(change_set)
}
