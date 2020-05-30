mod enums;
mod storage;
mod task;

use crate::enums::Priority;
use clap::{App, Arg};
use std::collections::HashSet;
use std::error::Error;
use storage::get_data;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut data, mut file) = get_data()?;

    let matches = App::new("Todo-CLI")
        .version("beta-0.1")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .about("A simple CLI app to track todos within the terminal")
        .arg(
            Arg::new("output")
                .about("Specify which output (default std-out)")
                .short('o')
                .default_value("terminal")
                .possible_values(&["cb", "clipboard", "t", "terminal"]),
        )
        .subcommand(App::new("add").about("Add new task"))
        .subcommand(
            App::new("list")
                .about("List all tasks that are critical and for today")
                .arg(
                    Arg::new("priority")
                        .about("Include priority in output")
                        .short('p')
                        .default_values(&["c", "t"])
                        .possible_values(&[
                            "t", "today", "c", "critical", "w", "week", "f", "future", "a", "all",
                        ])
                        .multiple(true),
                ),
        )
        .get_matches();

    if matches.is_present("add") {
        data.add_todo()?;
        data.save_data(&mut file)?;
    }

    let output = matches.value_of("output").unwrap();

    if let Some(ref matches) = matches.subcommand_matches("list") {
        let priority_matches = matches
            .values_of("priority")
            .unwrap()
            .collect::<Vec<&str>>();
        let priority: Vec<Priority> =
            if priority_matches.contains(&"a") || priority_matches.contains(&"all") {
                vec![
                    Priority::Critical,
                    Priority::Today,
                    Priority::Week,
                    Priority::Sometime,
                ]
            } else {
                let mut set = HashSet::new();
                priority_matches
                    .into_iter()
                    .map(Priority::from)
                    .for_each(|p| {
                        set.insert(p);
                    });
                set.into_iter().collect()
            };
        data.list_todos(output.into(), priority);
    }

    Ok(())
}
