use dialoguer::Select;

use crate::domain::Status;

pub(crate) fn get_state_input(existing_field: Status) -> Status {
    // @todo - doesn't seem to be a viable way to test Dialoguer?
    if cfg!(test) {
        return existing_field;
    };
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
