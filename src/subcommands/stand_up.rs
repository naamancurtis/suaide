use chrono::Local;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::io;

use diesel::prelude::*;

use crate::common::time::calculate_duration_from_timeframe;
use crate::domain::{Status, SuaideError, Task, Timeframe};
use crate::state::State;

pub fn app<'a>() -> App<'a, 'static> {
    App::new("standup")
        .about("Output your stand-up report")
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Provide additional information about each task"),
        )
}

pub fn handler<W: io::Write>(
    matches: &ArgMatches,
    state: &mut State<W>,
) -> Result<(), SuaideError> {
    let is_verbose = matches.is_present("verbose");
    let (today_start, today_end) =
        calculate_duration_from_timeframe(Local::now().date(), Timeframe::Today);
    let (yesterday_start, yesterday_end) =
        calculate_duration_from_timeframe(Local::now().date(), Timeframe::Yesterday);

    use crate::schema::suaide::dsl::{closed, opened, status, suaide};

    let mut today = suaide
        .filter(status.le(Status::InProgress as i16))
        .or_filter(status.le(Status::Closed as i16))
        .filter(closed.between(today_start, today_end))
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
