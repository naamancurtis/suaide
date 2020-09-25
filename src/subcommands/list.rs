use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};

use diesel::prelude::*;

use crate::common::{time::calculate_duration_from_timeframe, DATE_INPUT_LONG, DATE_INPUT_SHORT};
use crate::domain::{SuaideError, Task};

pub fn app() -> App<'static> {
    App::new("list")
        .about("List your tasks")
        .arg(
            Arg::with_name("timeframe")
                .index(1)
                .about("What timeframe would you like to list the tasks for?")
                .default_value("today")
                .possible_values(&["today", "yesterday", "week", "last-week", "month", "all"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("duration")
                .long("duration")
                .short('d')
                .conflicts_with("timeframe")
                .number_of_values(2)
                .about("Search for tasks between a specified duration")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("status")
                .long("status")
                .short('s')
                .about("Specify a status of the tasks you would like to list")
                .default_value("all")
                .possible_values(&["open", "closed", "all"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .about("Provide additional information about each task"),
        )
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
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

    use crate::schema::suaide::dsl::*;

    let mut results = suaide
        .filter(opened.between(start, end))
        .or_filter(closed.between(start, end))
        .order_by(closed.asc())
        .load::<Task>(&db_conn)?;

    results.sort();
    results.iter().for_each(|result| result.print(is_verbose));
    Ok(())
}

fn calculate_duration_from_dates(from: &str, to: &str) -> Result<(i64, i64), SuaideError> {
    let from = match NaiveDate::parse_from_str(from, DATE_INPUT_SHORT) {
        Ok(r) => r,
        Err(_) => NaiveDate::parse_from_str(from, DATE_INPUT_LONG)?,
    };
    let to = match NaiveDate::parse_from_str(to, DATE_INPUT_SHORT) {
        Ok(r) => r,
        Err(_) => NaiveDate::parse_from_str(to, DATE_INPUT_LONG)?,
    };
    let from = Local
        .ymd(from.year(), from.month(), from.day())
        .and_hms(0, 0, 1)
        .timestamp();
    let to = Local
        .ymd(to.year(), to.month(), to.day())
        .and_hms(23, 59, 59)
        .timestamp();
    Ok((from, to))
}
