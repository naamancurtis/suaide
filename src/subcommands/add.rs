use crate::schema::suaide;
use chrono::Local;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io::stdin;

use diesel::prelude::*;
use diesel::Insertable;

use crate::errors::SuaideError;

#[derive(Insertable)]
#[table_name = "suaide"]
struct AddItemStruct {
    ticket: Option<String>,
    description: String,
    opened: i64,
}

pub fn app() -> App<'static> {
    App::new("add")
        .about("Add new task")
        .arg(
            Arg::with_name("ticket_id")
                .long("ticket")
                .short('t')
                .about("Ticket identifier")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("description")
                .long("desc")
                .short('d')
                .about("Description")
                .takes_value(true),
        )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    let description: String;
    let ticket: Option<String>;

    if matches.is_present("description") {
        description = matches
            .value_of("description")
            .map(String::from)
            .expect("already checked string");
        ticket = matches.value_of("ticket_id").map(String::from);
    } else {
        let result = grab_input_from_user()?;
        description = result.0;
        ticket = result.1;
    }

    let task = AddItemStruct {
        description,
        ticket,
        opened: Local::now().timestamp(),
    };

    let _ = diesel::insert_into(suaide::table)
        .values(&task)
        .execute(&db_conn)?;
    println!("{}: {}", "Added task".green(), task.description);
    Ok(())
}

fn grab_input_from_user() -> Result<(String, Option<String>), SuaideError> {
    let mut description = String::new();

    println!("{}", "Enter your task description".italic());
    stdin().read_line(&mut description).unwrap();
    description = description
        .strip_suffix("\n")
        .ok_or_else(|| SuaideError::IncorrectArgs)?
        .to_string();

    println!("Add a ticket number? {}", "(press 'n' to skip)".italic());
    let mut temp_string = String::new();
    let mut ticket = None;
    stdin().read_line(&mut temp_string).unwrap();
    if temp_string != "n\n" {
        ticket = Some(
            temp_string
                .strip_suffix("\n")
                .ok_or_else(|| SuaideError::IncorrectArgs)?
                .to_string(),
        );
    }

    Ok((description, ticket))
}
