use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::domain::SuaideError;

lazy_static! {
    static ref DEFAULT_SUAIDE_PATH: &'static str = "~/.suaide";
    static ref DEFAULT_DB_PATH: &'static str = "~/.suaide/suaide.sqlite";
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub(crate) db_url: String,
    pub(crate) ticket_prefix: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.set_default("db_url", DEFAULT_DB_PATH.as_ref())?;
        s.set_default("ticket_prefix", "")?;

        let config_name =
            shellexpand::tilde(&format!("{}/settings.yml", DEFAULT_SUAIDE_PATH.to_string()))
                .to_string();

        if cfg!(not(test)) {
            // Read Config Settings
            s.merge(File::with_name(&config_name).required(false))?;

            // Overwrite with environment variables
            s.merge(Environment::with_prefix("SUAIDE"))?;
        }
        let db_url = s.get_str("db_url")?;
        let db_url = shellexpand::tilde(&db_url).to_string();
        verify_or_setup_folder_structure(db_url.clone()).map_err(|_| {
            ConfigError::NotFound("Config directory was not able to be initialized".to_string())
        })?;
        s.set("db_url", db_url)?;

        s.try_into()
    }

    pub fn generate_ticket_id(&self, ticket: String) -> String {
        format!("{}{}", self.ticket_prefix, ticket)
    }
}

fn verify_or_setup_folder_structure(path: String) -> Result<(), SuaideError> {
    let path = Path::new(&path);
    if path.exists() {
        return Ok(());
    }

    let path_parent = path.parent().unwrap();
    fs::create_dir_all(path_parent)?;

    Ok(())
}
