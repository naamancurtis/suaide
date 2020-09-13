use chrono::prelude::*;
use chrono::Duration;
use colored::Colorize;
use dialoguer::{Input, Select};

use crate::enums::{Status, Timeframe};
use crate::errors::SuaideError;
use crate::task::Task;

use diesel::prelude::*;

const NO_EDIT_PREFIX: &str = "Enter";
const EDIT_PREFIX: &str = "Edit";

pub(crate) fn get_input(key: &str, existing_field: Option<String>) -> Result<String, SuaideError> {
    let is_edit = existing_field.is_some();
    let prefix = if is_edit { EDIT_PREFIX } else { NO_EDIT_PREFIX };
    let mut input = Input::<String>::new();
    let input = input
        .with_prompt(format!("{} task {}", prefix, key))
        .allow_empty(false);

    if let Some(text) = existing_field {
        input.with_initial_text(text);
    }
    Ok(input.interact()?)
}

pub(crate) fn get_optional_input(
    key: &str,
    existing_field: Option<String>,
) -> Result<Option<String>, SuaideError> {
    let is_edit = existing_field.is_some();
    let prefix = if is_edit { EDIT_PREFIX } else { NO_EDIT_PREFIX };
    let mut input = Input::<String>::new();
    let input = input
        .with_prompt(format!(
            "{} task {} {}",
            prefix,
            key,
            "(Enter to skip)".italic()
        ))
        .allow_empty(true);

    if let Some(text) = existing_field {
        input.with_initial_text(text);
    }
    let final_input = input.interact()?;
    if final_input.is_empty() {
        return Ok(None);
    }
    Ok(Some(final_input))
}

pub(crate) fn get_state_input(existing_field: Status) -> Status {
    let options = vec![
        Status::Open,
        Status::InProgress,
        Status::Closed,
        Status::Cancelled,
    ];
    let option_text: Vec<String> = options.iter().map(|s| s.to_string()).collect();
    let current = options
        .iter()
        .position(|s| *s == existing_field)
        .expect("Status should always be present");
    let select = Select::new()
        .items(&option_text)
        .default(current)
        .interact()
        .unwrap();
    options[select]
}

pub(crate) fn get_task(task: &str, db_conn: &SqliteConnection) -> Result<Task, SuaideError> {
    use crate::schema::suaide::dsl::*;

    if let Ok(mut result) = suaide
        .filter(ticket.eq(Some(task)))
        .limit(1)
        .load::<Task>(db_conn)
    {
        if result.len() == 1 {
            let found_task = result.pop().unwrap();
            return Ok(found_task);
        }
    }
    if let Ok(num) = task.parse::<i32>() {
        if let Ok(task) = suaide.find(num).first(db_conn) {
            return Ok(task);
        }
    }
    Err(SuaideError::NotFound)
}

pub(crate) fn calculate_duration_from_timeframe(timeframe: Timeframe) -> (i64, i64) {
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
