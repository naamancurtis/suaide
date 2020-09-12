use clap::{App, Arg, ArgMatches};

use diesel::prelude::*;

use crate::common::{get_input, get_optional_input, get_state_input, get_task};
use crate::enums::Status;
use crate::errors::SuaideError;
use crate::task::Task;

pub fn app() -> App<'static> {
    App::new("edit").about("Edit a task").arg(
        Arg::with_name("task")
            .index(1)
            .about("Mark this task as closed")
            .takes_value(true),
    )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    if let Some(task) = matches.value_of("task") {
        let mut task = get_task(task, &db_conn)?;
        task = grab_input_from_user(task)?;
        let change_set = task.into_changeset();

        use crate::schema::suaide::dsl::*;

        diesel::update(suaide.find(task.id))
            .set(change_set)
            .execute(&db_conn)?;

        task.print();
        return Ok(());
    }
    Err(SuaideError::IncorrectArgs)
}

fn grab_input_from_user(mut task: Task) -> Result<Task, SuaideError> {
    task.description = get_input("description", Some(task.description))?;
    task.ticket = get_optional_input("ID", task.ticket)?;
    let status = get_state_input(task.status.into());
    task.status = status as i16;
    if task.closed.is_some() && status != Status::Closed {
        task.closed = None;
    }
    Ok(task)
}
