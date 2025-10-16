use std::{env, path::PathBuf};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub rclone_path: String,
    pub custom_storages: Vec<String>,
}

// macOS: ~/Library/Application Support/<app>/<config_name>.toml
// Linux: ~/.config/<app>/<config_name>.toml
// Windows: %APPDATA%\<app>\<config_name>.toml
pub fn user_conf() -> UserConfig {
    confy::load(env!("APP_NAME"), env!("APP_CONFIG_NAME")).unwrap_or_else(|e| {
        error!("Failed to load config: {}. Using default.", e);
        UserConfig::default()
    })
}

pub fn user_conf_path() -> Result<PathBuf> {
    let user_conf_path = confy::get_configuration_file_path(env!("APP_NAME"), env!("APP_CONFIG_NAME"))?;
    Ok(user_conf_path)
}