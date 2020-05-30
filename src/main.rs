mod storage;
mod task;

use clap::App;
use std::error::Error;
use storage::get_data;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut data, mut file) = get_data()?;

    let matches = App::new("Todo-CLI")
        .version("beta-0.1")
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
