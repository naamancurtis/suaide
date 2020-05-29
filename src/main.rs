use chrono::{NaiveDateTime, Utc};
use clap::{App, Arg};
use dialoguer::{Input, Select};
use std::io::Error;

#[derive(Debug)]
pub struct Todo {
    id: u32,
    message: String,
    importance: Importance,
    issue_number: Option<String>,
    opened: NaiveDateTime,
    closed: Option<NaiveDateTime>,
}

impl Todo {
    fn new(importance: Importance, message: String, issue_number: Option<String>) -> Self {
        Self {
            id: 1,
            message,
            importance,
            issue_number,
            opened: Utc::now().naive_utc(),
            closed: None,
        }
    }
}

#[derive(Debug)]
enum Importance {
    Critical,
    Today,
    Week,
    Sometime,
}

impl ToString for Importance {
    fn to_string(&self) -> String {
        match self {
            Importance::Critical => String::from("Absolultely Critical"),
            Importance::Sometime => String::from("Sometime in the future"),
            Importance::Today => String::from("Needs to be done today"),
            Importance::Week => String::from("Needs to be done sometime this week"),
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = App::new("Todo-CLI")
        .version("BETA")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .about("A simple CLI app to track todos within the terminal")
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .value_name("ADD")
                .takes_value(false)
                .about("Add a new task or todo"),
        )
        .get_matches();

    if matches.is_present("add") {
        println!("Matched Add");

        let mut priority_options = vec![
            Importance::Today,
            Importance::Critical,
            Importance::Week,
            Importance::Sometime,
        ];

        let priority = Select::new()
            .with_prompt("Please select the priority of the task")
            .items(&priority_options)
            .default(0)
            .interact()?;

        let priority = priority_options.swap_remove(priority);

        let issue_number = match Input::<String>::new()
            .allow_empty(true)
            .with_prompt("Is this part of an issue (leave blank if not)")
            .interact()
        {
            Ok(issue_number) if !issue_number.is_empty() => Some(issue_number),
            Ok(_) => None,
            Err(e) => return Err(e),
        };

        let message = Input::<String>::new()
            .with_prompt("What is the task?")
            .interact()?;

        let todo = Todo::new(priority, message, issue_number);
        println!("{:?}", todo);
    }

    Ok(())
}
