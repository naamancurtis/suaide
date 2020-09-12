use clap::{App, Arg, ArgMatches};

use chrono::prelude::*;
use diesel::prelude::*;

use crate::common::{get_state_input, get_task};
use crate::enums::Status;
use crate::errors::SuaideError;
use crate::task::{Task, TaskChangeSet};

pub fn app() -> App<'static> {
    App::new("status")
        .about("Change the status of a task")
        .arg(
            Arg::with_name("task")
                .index(1)
                .about("Update the status of this task")
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
                ])
                .takes_value(true),
        )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    if let Some(task_id) = matches.value_of("task") {
        let task = get_task(task_id, &db_conn)?;
        let change_set = if let Some(state) = matches.value_of("state") {
            let updated_status = Status::from(state);
            generate_change_set(&task, updated_status)?
        } else {
            let updated_status = get_state_input(task.status.into());
            generate_change_set(&task, updated_status)?
        };

        use crate::schema::suaide::dsl::*;

        diesel::update(suaide.find(task.id))
            .set(change_set)
            .execute(&db_conn)?;

        let task = get_task(task_id, &db_conn)?;
        task.print();
        return Ok(());
    }
    Err(SuaideError::IncorrectArgs)
}

fn generate_change_set(task: &Task, status: Status) -> Result<TaskChangeSet, SuaideError> {
    let mut change_set = TaskChangeSet::default();
    change_set.set_status(task, status);
    if task.closed.is_some() && status != Status::Closed {
        change_set.set_closed(task, None);
    }
    if status == Status::Closed {
        change_set.set_closed(task, Some(Local::now().timestamp()))
    }
    Ok(change_set)
}
