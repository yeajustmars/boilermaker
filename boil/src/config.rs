use std::fs;
use std::path::Path;

use color_eyre::eyre::{Error, Result};
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::Deserialize;
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
pub fn get_system_config_path(config_path: Option<&Path>) -> Result<Option<&Path>> {
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

// TODO: return BoilermakerSysConfig struct for get_system_config (instead of generic toml::Value)
// pub struct BoilermakerSysConfig {

// }

//TODO: add ability for config to be in YAML as well as TOML
//TODO: remove Option<&Path>.
#[tracing::instrument]
pub fn get_system_config(config_path: Option<&Path>) -> Result<Value> {
    if let Some(path) = get_system_config_path(config_path)? {
        let config_content = fs::read_to_string(path)?;
        let config: toml::Value = toml::from_str(&config_content)?;
        Ok(config)
    } else {
        Ok(make_default_config())
    }
}

#[tracing::instrument]
pub fn get_template_config(config_path: &Path) -> Result<BoilermakerConfig> {
    if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        let config: BoilermakerConfig = toml::from_str(&config_content)?;
        Ok(config)
    } else {
        Err(color_eyre::eyre::eyre!(
            "❗ Config file not found at `{}`.",
            config_path.display()
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct BoilermakerConfig {
    pub boilermaker: BoilermakerConfigRoot,
}

#[derive(Debug, Deserialize)]
pub struct BoilermakerConfigRoot {
    pub project: BoilermakerConfigProject,
    pub variables: Option<toml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct BoilermakerConfigProject {
    pub name: String,
    pub repository: String,
    pub subdir: Option<String>,
    pub version: Option<String>,
    pub default_lang: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub website: Option<String>,
}
