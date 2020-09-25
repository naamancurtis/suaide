use colored::Colorize;
use dialoguer::{Input, Select};

use crate::common::{ADD_PREFIX, EDIT_PREFIX};
use crate::domain::{Status, SuaideError};

pub(crate) fn get_input(key: &str, existing_field: Option<String>) -> Result<String, SuaideError> {
    let is_edit = existing_field.is_some();
    let prefix = if is_edit { EDIT_PREFIX } else { ADD_PREFIX };
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
    let prefix = if is_edit { EDIT_PREFIX } else { ADD_PREFIX };
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
