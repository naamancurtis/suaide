use diesel::prelude::*;

use crate::domain::{SuaideError, Task};

pub(crate) fn get_task(task: &str, db_conn: &SqliteConnection) -> Result<Task, SuaideError> {
    use crate::schema::suaide::dsl::*;

    if let Ok(mut result) = suaide
        .filter(ticket.eq(Some(task)))
        .limit(1)
        .load::<Task>(db_conn)
    {
        if result.len() == 1 {
            let found_task = result.pop().unwrap();
            return Ok(found_task);
        }
    }
    if let Ok(num) = task.parse::<i32>() {
        if let Ok(task) = suaide.find(num).first(db_conn) {
            return Ok(task);
        }
    }
    Err(SuaideError::NotFound)
}
