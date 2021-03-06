use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use std::io;

use diesel::prelude::*;

use crate::common::time::{calculate_duration_from_dates, calculate_duration_from_timeframe};
use crate::domain::{SuaideError, Task};
use crate::state::State;

pub fn app<'a>() -> App<'a, 'static> {
    App::new("list")
        .about("List your tasks")
        .arg(
            Arg::with_name("timeframe")
                .index(1)
                .help("What timeframe would you like to list the tasks for?")
                .conflicts_with("duration")
                .default_value("today")
                .possible_values(&["today", "yesterday", "week", "lastweek", "month", "all"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("duration")
                .long("duration")
                .short("d")
                .conflicts_with("timeframe")
                .number_of_values(2)
                .next_line_help(true)
                .long_help(
                    "Search for all tasks between two dates. \nDates should be provided in one of the following formats \"YYYY-MM-DD\" or \"DD mmm YYYY\"\nExample: 2020-01-01 or 1 Jan 2020\n",
                )
                .takes_value(true),
        )
        // @todo
        // .arg(
        //     Arg::with_name("status")
        //         .long("status")
        //         .short('s')
        //         .about("Specify a status of the tasks you would like to list")
        //         .default_value("all")
        //         .possible_values(&["open", "closed", "all"])
        //         .takes_value(true),
        // )
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
    let (mut start, mut end): (i64, i64) = (0, Local::now().timestamp());
    if let Some(duration_iter) = matches.values_of("duration") {
        let duration: Vec<&str> = duration_iter.collect();
        if duration.len() != 2 {
            return Err(SuaideError::IncorrectArgs);
        }
        let result = calculate_duration_from_dates(duration[0], duration[1])?;
        start = result.0;
        end = result.1;
    } else {
        let tf = matches.value_of("timeframe").expect("has default value");
        if tf != "all" {
            let result = calculate_duration_from_timeframe(Local::now().date(), tf.into());
            start = result.0;
            end = result.1;
        }
    };

    use crate::schema::suaide::dsl::{closed, opened, suaide};

    let mut results = suaide
        .filter(opened.between(start, end))
        .or_filter(closed.between(start, end))
        .order_by(closed.asc())
        .load::<Task>(state.get_conn())?;

    results.sort();
    results.iter().for_each(|result| result.print(is_verbose));
    Ok(())
}
