use chrono::prelude::*;
use chrono::Duration;
use clap::{App, Arg, ArgMatches};
use colored::Colorize;

use diesel::prelude::*;

use crate::enums::Timeframe;
use crate::errors::SuaideError;
use crate::task::Task;

const DATE_FORMAT: &str = "%Y-%m-%d";
const WRITTEN_DATE_FORMAT: &str = "%e %b %Y";

pub fn app() -> App<'static> {
    App::new("list")
        .about("List your tasks")
        .arg(
            Arg::with_name("timeframe")
                .index(1)
                .about("What timeframe would you like to list the tasks for?")
                .default_value("today")
                .possible_values(&["today", "yesterday", "week", "last-week", "month"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("duration")
                .long("duration")
                .short('d')
                .conflicts_with("timeframe")
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
}

pub fn handler(matches: &ArgMatches, db_conn: SqliteConnection) -> Result<(), SuaideError> {
    let status = matches.value_of("status").expect("has default value");
    let (start, end): (i64, i64);
    if let Some(duration_iter) = matches.values_of("duration") {
        let duration: Vec<&str> = duration_iter.collect();
        if duration.len() != 2 {
            return Err(SuaideError::IncorrectArgs);
        }
        let result = calculate_duration_from_dates(duration[0], duration[1])?;
        start = result.0;
        end = result.1;
    } else {
        let timeframe = matches.value_of("timeframe").expect("has default value");
        let result = calculate_duration_from_timeframe(timeframe.into());
        start = result.0;
        end = result.1;
    };

    use crate::schema::suaide::dsl::*;
    let mut results = suaide
        .filter(opened.gt(start))
        .or_filter(closed.gt(start))
        .filter(opened.lt(end))
        .or_filter(closed.lt(end))
        .order_by(closed.asc())
        .load::<Task>(&db_conn)?;
    results.sort();
    results.iter().for_each(|result| result.list());
    Ok(())
}

fn calculate_duration_from_timeframe(timeframe: Timeframe) -> (i64, i64) {
    let base_date = Local::now().date();
    let now = Local.ymd(base_date.year(), base_date.month(), base_date.day());
    let now_i64 = now.and_hms(23, 59, 59).timestamp();
    match timeframe {
        Timeframe::Today => (now.and_hms(0, 0, 1).timestamp(), now_i64),
        Timeframe::Yesterday => (
            (now.and_hms(0, 0, 1) - Duration::days(1)).timestamp(),
            (now.and_hms(23, 59, 59) - Duration::days(1)).timestamp(),
        ),
        Timeframe::Week => (
            Local
                .isoywd(base_date.year(), base_date.iso_week().week(), Weekday::Mon)
                .and_hms(0, 0, 1)
                .timestamp(),
            now_i64,
        ),
        Timeframe::LastWeek => {
            let iso_date = base_date - Duration::days(7);
            (
                Local
                    .isoywd(iso_date.year(), iso_date.iso_week().week(), Weekday::Mon)
                    .and_hms(0, 0, 1)
                    .timestamp(),
                Local
                    .isoywd(iso_date.year(), iso_date.iso_week().week(), Weekday::Fri)
                    .and_hms(23, 59, 59)
                    .timestamp(),
            )
        }
        Timeframe::Month => (
            Local
                .ymd(base_date.year(), base_date.month(), 1)
                .and_hms(0, 0, 1)
                .timestamp(),
            now_i64,
        ),
    }
}

fn calculate_duration_from_dates(from: &str, to: &str) -> Result<(i64, i64), SuaideError> {
    let from = match Local.datetime_from_str(from, DATE_FORMAT) {
        Ok(r) => r,
        Err(_) => Local.datetime_from_str(from, WRITTEN_DATE_FORMAT)?,
    };
    let to = match Local.datetime_from_str(to, DATE_FORMAT) {
        Ok(r) => r,
        Err(_) => Local.datetime_from_str(to, WRITTEN_DATE_FORMAT)?,
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
