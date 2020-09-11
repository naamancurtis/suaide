use clap::{App, Arg, ArgMatches};
use colored::Colorize;

use diesel::prelude::*;

use crate::errors::SuaideError;

pub fn app() -> App<'static> {
    App::new("remove").about("Delete a tasks").arg(
        Arg::with_name("ticket")
            .index(1)
            .about("What ticket would you like to delete")
            .takes_value(true),
    )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    if let Some(task) = matches.value_of("ticket") {
        use crate::schema::suaide::dsl::*;
        let ticket_name = format!("%{}%", task);

        if let Ok(num) = task.parse::<i32>() {
            diesel::delete(suaide.filter(ticket.like(Some(ticket_name)).or(id.eq(num))))
                .execute(&db_conn)?;
        } else {
            diesel::delete(suaide.filter(ticket.like(Some(ticket_name)))).execute(&db_conn)?;
        }

        println!("[{}]: Task {}", "Removed".truecolor(224, 108, 117), task);
        return Ok(());
    }
    Err(SuaideError::NotFound)
}
