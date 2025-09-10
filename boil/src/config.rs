use std::fs;
use std::path::Path;

use color_eyre::eyre::{Error, Result};
use dirs::home_dir;
use lazy_static::lazy_static;
use toml::{Value, map::Map as TomlMap};
use tracing::{info, warn};

lazy_static! {
    pub static ref SYS_CONFIG_FILE: String = format!(
        "{}/.config/boilermaker/boilermaker.toml",
        home_dir().unwrap().to_str().unwrap()
    );
}

//TODO: add default configuration for boil cmd
#[tracing::instrument]
pub fn make_default_config() -> Value {
    let mut config = TomlMap::new();
    config.insert("log_level".to_string(), Value::String("INFO".into()));
    Value::Table(config)
}

//TODO: add ability for config to be in YAML as well as TOML
#[tracing::instrument]
pub fn get_config_path(config_path: Option<&Path>) -> Result<Option<&Path>> {
    if let Some(path) = config_path {
        if !path.exists() {
            return Err(Error::msg(format!(
                "❗ Provided config file not found at `{}`.",
                path.display()
            )));
        } else {
            info!(" Using provided config file: `{}`.", path.display());
            Ok(Some(path))
        }
    } else if fs::exists(SYS_CONFIG_FILE.as_str()).unwrap() {
        let path = Path::new(SYS_CONFIG_FILE.as_str());
        info!("Using system config file: `{}`.", path.display());
        Ok(Some(path))
    } else {
        info!(" Using default config.");
        Ok(None)
    }
}

//TODO: add ability for config to be in YAML as well as TOML
#[tracing::instrument]
pub fn get_config(config_path: Option<&Path>) -> Result<Value> {
    if let Some(path) = get_config_path(config_path)? {
        let config_content = fs::read_to_string(path)?;
        let config: toml::Value = toml::from_str(&config_content)?;
        Ok(config)
    } else {
        Ok(make_default_config())
    }
}
