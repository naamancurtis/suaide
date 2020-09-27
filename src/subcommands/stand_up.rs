use chrono::Local;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io;

use diesel::prelude::*;

use crate::common::time::calculate_duration_from_timeframe;
use crate::domain::{Status, SuaideError, Task, Timeframe};
use crate::state::State;

pub fn app() -> App<'static> {
    App::new("standup")
        .about("Output your stand-up report")
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .about("Provide additional information about each task"),
        )
}

pub fn handler<R: io::BufRead, W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<R, W>,
) -> Result<(), SuaideError> {
    let is_verbose = matches.is_present("verbose");
    let (yesterday_start, yesterday_end) =
        calculate_duration_from_timeframe(Local::now().date(), Timeframe::Yesterday);

    use crate::schema::suaide::dsl::{closed, opened, status, suaide};

    let mut today = suaide
        .filter(status.le(Status::InProgress as i16))
        .load::<Task>(state.get_conn())?;

    let mut yesterday = suaide
        .filter(status.eq(Status::Closed as i16))
        .filter(closed.between(yesterday_start, yesterday_end))
        .or_filter(status.eq(Status::InProgress as i16))
        .filter(opened.lt(yesterday_end))
        .load::<Task>(state.get_conn())?;

    yesterday.sort();
    today.sort();

    println!("=== {} ===", "Yesterday".bold());
    yesterday.iter().for_each(|result| result.print(is_verbose));
    println!();

    println!("=== {} ===", "Today".bold());
    today.iter().for_each(|result| result.print(is_verbose));
    println!();

    Ok(())
}
