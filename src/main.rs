use chrono::{NaiveDateTime, Utc};
use clap::App;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

const DATA_DIR: &str = "./.task-tracker";
const DATA_PATH: &str = "./.task-tracker/data.json";

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    open: HashMap<Priority, Vec<Task>>,
    closed: Vec<Task>,
    next_id: u32,
}

impl Data {
    fn new() -> Self {
        let mut open = HashMap::new();
        open.insert(Priority::Critical, Vec::new());
        open.insert(Priority::Today, Vec::new());
        open.insert(Priority::Week, Vec::new());
        open.insert(Priority::Sometime, Vec::new());
        Self {
            open,
            closed: Vec::new(),
            next_id: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    message: String,
    priority: Priority,
    issue_number: Option<String>,
    opened: NaiveDateTime,
    closed: Option<NaiveDateTime>,
}

impl Task {
    fn new(id: u32, priority: Priority, message: String, issue_number: Option<String>) -> Self {
        Self {
            id,
            message,
            priority,
            issue_number,
            opened: Utc::now().naive_utc(),
            closed: None,
        }
    }

    fn open(&mut self) {
        self.closed = None;
    }

    fn complete(&mut self) {
        self.closed = Some(Utc::now().naive_utc());
    }

    fn is_complete(&mut self) -> bool {
        self.closed.is_some()
    }

    fn get_priority(&self) -> &Priority {
        &self.priority
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialOrd, PartialEq, Hash)]
enum Priority {
    Critical,
    Today,
    Week,
    Sometime,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Critical => String::from("Absolultely Critical"),
            Priority::Today => String::from("Needs to be done today"),
            Priority::Week => String::from("Needs to be done sometime this week"),
            Priority::Sometime => String::from("Sometime in the future"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut data, mut file) = get_data()?;

    let matches = App::new("Todo-CLI")
        .version("β0.1")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .about("A simple CLI app to track todos within the terminal")
        .subcommand(App::new("add").about("Add new task"))
        .subcommand(App::new("list").about("List tasks"))
        .get_matches();

    if matches.is_present("add") {
        data.add_todo()?;
        data.save_data(&mut file)?;
    }

    Ok(())
}

fn get_data() -> Result<(Data, File), Box<dyn Error>> {
    let file = get_file_handle()?;
    let reader = BufReader::new(&file);

    match serde_json::from_reader(reader) {
        Ok(data) => Ok((data, file)),
        Err(e) => Err(Box::new(e)),
    }
}

fn get_file_handle() -> Result<File, Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let mut permissions = std::fs::metadata(&current_dir)?.permissions();
    if permissions.readonly() {
        permissions.set_readonly(false);
        fs::set_permissions(current_dir, permissions)?;
    };

    let file = match fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATA_PATH)
    {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            create_dir(DATA_DIR)?;
            let mut new_file = File::create(DATA_PATH)
                .unwrap_or_else(|e| panic!("Unable to create data file: {:?}", e));

            let data = Data::new();
            data.save_data(&mut new_file)?;
            return get_file_handle();
        }

        Err(e) => return Err(Box::new(e)),
    };
    Ok(file)
}

impl Data {
    fn save_data(&self, handle: &mut File) -> Result<(), Box<dyn Error>> {
        handle.seek(std::io::SeekFrom::Start(0));
        serde_json::to_writer_pretty(handle, self)?;
        Ok(())
    }

    fn add_todo(&mut self) -> Result<(), Box<dyn Error>> {
        let mut priority_options = vec![
            Priority::Today,
            Priority::Critical,
            Priority::Week,
            Priority::Sometime,
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
            Err(e) => return Err(Box::new(e)),
        };

        let message = Input::<String>::new()
            .with_prompt("What is the task?")
            .interact()?;

        let task = Task::new(self.next_id, priority, message, issue_number);
        self.next_id += 1;
        if let Some(bucket) = self.open.get_mut(task.get_priority()) {
            bucket.push(task);
        }

        Ok(())
    }
}
