use std::{
    collections::HashMap,
    fs,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Error, Result, eyre};
use dirs::home_dir;
use lazy_static::lazy_static;
use minijinja::value::Value as JinjaValue;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

lazy_static! {
    pub static ref SYS_CONFIG_FILE: String = format!(
        "{}/.config/boilermaker/boilermaker.toml",
        home_dir().unwrap().to_str().unwrap()
    );
    pub static ref DEFAULT_LOCAL_DB_PATH: PathBuf = make_boilermaker_local_db_path().unwrap();
    pub static ref DEFAULT_LOCAL_DB_PATH_STRING: String = DEFAULT_LOCAL_DB_PATH
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
    pub static ref DEFAULT_WEBSITE_DATABASE_PATH: PathBuf =
        PathBuf::from("/var/lib/boilermaker/boilermaker.db");
    pub static ref DEFAULT_WEBSITE_DATABASE_PATH_STRING: String = DEFAULT_WEBSITE_DATABASE_PATH
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
}

//TODO: add default configuration for boil cmd
#[tracing::instrument]
pub fn make_default_config() -> SysConfig {
    SysConfig {
        log_level: Some("INFO".to_string()),
        sources: None,
    }
}

//TODO: add ability for config to be in YAML as well as TOML
#[tracing::instrument]
pub fn get_system_config_path(config_path: Option<&Path>) -> Result<Option<&Path>> {
    if let Some(path) = config_path {
        if !path.exists() {
            Err(Error::msg(format!(
                "❗ Provided config file not found at `{}`.",
                path.display()
            )))
        } else {
            info!(" Using provided config file: `{}`.", path.display());
            Ok(Some(path))
        }
    } else if fs::exists(SYS_CONFIG_FILE.as_str()).unwrap() {
        let path = Path::new(SYS_CONFIG_FILE.as_str());
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

// TODO: add default_project_dir and override in global config
#[derive(Debug, Deserialize)]
pub struct SysConfig {
    pub log_level: Option<String>,
    pub sources: Option<Vec<HashMap<String, String>>>,
}

//TODO: add ability for config to be in YAML as well as TOML
//TODO: remove Option<&Path>.
#[tracing::instrument]
pub fn get_system_config(config_path: Option<&Path>) -> Result<SysConfig> {
    if let Some(path) = get_system_config_path(config_path)? {
        let config_content = fs::read_to_string(path)?;
        // let config: toml::Value = toml::from_str(&config_content)?;
        let config: SysConfig = toml::from_str(&config_content)?;
        Ok(config)
    } else {
        Ok(make_default_config())
    }
}

#[tracing::instrument]
pub fn make_boilermaker_local_db_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Can't find home directory"))?;
    let local_db_dir = home_dir.join(".boilermaker");

    fs::create_dir_all(local_db_dir)?;

    let local_db_path = home_dir.join(".boilermaker").join("local_db.db");

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&local_db_path)
    {
        Ok(_) => Ok(local_db_path),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Ok(local_db_path)
            } else {
                Err(eyre!("💥 Failed to create local cache file: {}", e))
            }
        }
    }
}

#[tracing::instrument]
pub fn get_template_config_text(template_path: &Path) -> Result<String> {
    let config_path = template_path.join("boilermaker.toml");

    if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        Ok(config_content)
    } else {
        Err(color_eyre::eyre::eyre!(
            "💥 Config file not found at `{}`.",
            config_path.display()
        ))
    }
}

#[tracing::instrument]
pub fn template_config_text_to_config(text: &str) -> Result<TemplateConfig> {
    Ok(toml::from_str(text)?)
}

#[tracing::instrument]
pub fn get_template_config(template_path: &Path) -> Result<TemplateConfig> {
    let text = get_template_config_text(template_path)?;
    let config = template_config_text_to_config(&text)?;

    Ok(config)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub project: TemplateConfigProject,
    pub variables: Option<JinjaValue>,
}

// TODO: add all remaining fields
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateConfigProject {
    // name, version required
    pub name: String,
    pub version: String,

    // Git info
    pub repository: String,
    pub default_branch: Option<String>,
    pub default_subdir: Option<String>,
    pub default_lang: Option<String>,

    // Metadata
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub website: Option<String>,
}
