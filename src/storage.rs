use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

use crate::enums::*;
use crate::task::Task;

const DATA_DIR: &str = "./.task-tracker";
const DATA_PATH: &str = "./.task-tracker/data.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
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

impl Data {
    pub fn save_data(&self, handle: &mut File) -> Result<(), Box<dyn Error>> {
        handle.seek(std::io::SeekFrom::Start(0))?;
        serde_json::to_writer_pretty(handle, self)?;
        Ok(())
    }

    pub fn add_todo(&mut self) -> Result<(), Box<dyn Error>> {
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

    pub fn list_todos(&self, output: Output, priority: Vec<Priority>) {
        use colored::*;

        if output == Output::Terminal {
            priority.into_iter().for_each(|p| {
                if let Some(tasks) = self.open.get(&p) {
                    if tasks.len() > 0 {
                        println!("======================================================");
                        println!("{}", p.to_string().color("yellow").bold());
                        println!("======================================================");
                        tasks.iter().for_each(|task| println!("{}", task));
                    }
                }
            })
        }
    }
}

pub fn get_data() -> Result<(Data, File), Box<dyn Error>> {
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
