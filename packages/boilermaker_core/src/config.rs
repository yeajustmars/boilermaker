use std::{
    collections::HashMap,
    fs,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};
use dirs::home_dir;
use lazy_static::lazy_static;
use minijinja::value::Value as JinjaValue;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

lazy_static! {
    pub static ref DEFAULT_CONFIG_FILE: String = format!(
        "{}/.config/boilermaker/boilermaker.toml",
        home_dir().unwrap().to_str().unwrap()
    );
    pub static ref DEFAULT_ETC_SYS_CONFIG_FILE: String =
        "/etc/boilermaker/boilermaker.toml".to_string();
    pub static ref DEFAULT_LOG_LEVEL: String = "INFO".to_string();
    pub static ref DEFAULT_TEMPLATE_DIR: PathBuf = get_template_base_dir(None).unwrap();
    pub static ref DEFAULT_TEMPLATE_DIR_STRING: String =
        DEFAULT_TEMPLATE_DIR.as_path().to_str().unwrap().to_string();
    pub static ref DEFAULT_LOCAL_DB_PATH: PathBuf = make_local_db_path(None).unwrap();
    pub static ref DEFAULT_LOCAL_DB_PATH_STRING: String = DEFAULT_LOCAL_DB_PATH
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
    pub static ref DEFAULT_ETC_DB_PATH: PathBuf =
        PathBuf::from("/var/lib/boilermaker/boilermaker.db");
    pub static ref DEFAULT_ETC_DB_PATH_STRING: String =
        DEFAULT_ETC_DB_PATH.as_path().to_str().unwrap().to_string();
}

//TODO: add ability for config to be in YAML as well as TOML
#[tracing::instrument]
pub fn get_system_config_path(config_path: Option<&Path>) -> Result<Option<&Path>> {
    if let Some(path) = config_path {
        if !path.exists() {
            Err(eyre!(
                "❗ Provided config file not found at `{}`.",
                path.display()
            ))
        } else {
            info!(" Using provided config file: `{}`.", path.display());
            Ok(Some(path))
        }
    } else {
        let valid_config_paths = [
            Path::new(DEFAULT_CONFIG_FILE.as_str()),
            Path::new(DEFAULT_ETC_SYS_CONFIG_FILE.as_str()),
        ];

        for path in &valid_config_paths {
            if path.exists() {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }
}

//TODO: add ability for config to be in YAML as well as TOML
//TODO: remove Option<&Path>.
#[tracing::instrument]
pub fn get_system_config(config_path: Option<&Path>) -> Result<SysConfig> {
    if let Some(path) = get_system_config_path(config_path)? {
        let config_content = fs::read_to_string(path)?;
        let mut config: SysConfig = toml::from_str(&config_content)?;
        config.db_path = expand_tilde(&config.db_path)
            .ok_or_else(|| {
                eyre!(
                    "💥 Failed to expand db_path in config: `{}`",
                    config.db_path
                )
            })?
            .to_str()
            .unwrap()
            .to_string();
        config.template_dir = expand_tilde(&config.template_dir)
            .ok_or_else(|| {
                eyre!(
                    "💥 Failed to expand template_dir in config: `{}`",
                    config.template_dir
                )
            })?
            .to_str()
            .unwrap()
            .to_string();
        Ok(config)
    } else {
        Ok(make_default_config())
    }
}

//TODO: add default configuration for boil cmd
#[tracing::instrument]
pub fn make_default_config() -> SysConfig {
    SysConfig {
        db_path: DEFAULT_LOCAL_DB_PATH_STRING.clone(),
        template_dir: DEFAULT_TEMPLATE_DIR_STRING.clone(),
        log_level: DEFAULT_LOG_LEVEL.clone(),
        sources: None,
    }
}

// TODO: add default_project_dir and override in global config
#[derive(Clone, Debug, Deserialize)]
pub struct SysConfig {
    pub db_path: String,
    pub template_dir: String,
    pub log_level: String,
    pub sources: Option<Vec<HashMap<String, String>>>,
}

impl From<SysConfig> for HashMap<String, String> {
    fn from(config: SysConfig) -> Self {
        config.to_hashmap()
    }
}

// TODO: implement sources for SysConfig.to_hashmap()
impl SysConfig {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("db_path".to_string(), self.db_path.to_string());
        map.insert("template_dir".to_string(), self.template_dir.to_string());
        map.insert("log_level".to_string(), self.log_level.to_string());
        map
    }
}

#[tracing::instrument]
pub fn make_local_db_path(config: Option<SysConfig>) -> Result<PathBuf> {
    let (db_dir, db_path) = if let Some(cfg) = config {
        let db_path = PathBuf::from(cfg.db_path);
        let db_dir = db_path
            .parent()
            .ok_or_else(|| eyre!("Invalid db_path in config: `{}`", db_path.display()))?
            .to_path_buf();
        (db_dir, db_path)
    } else {
        let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Can't find home directory"))?;
        let db_dir = home_dir.join(".boilermaker");
        let db_path = db_dir.join("boilermaker.db");
        (db_dir, db_path)
    };

    if !db_dir.exists() {
        fs::create_dir_all(&db_dir)?;
    }

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&db_path)
    {
        Ok(_) => Ok(db_path),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Ok(db_path)
            } else {
                Err(eyre!("💥 Failed to create local DB file: {}", e))
            }
        }
    }
}

#[tracing::instrument]
pub fn get_template_base_dir(config: Option<SysConfig>) -> Result<PathBuf> {
    let template_dir = if let Some(cfg) = config {
        let path = PathBuf::from(cfg.template_dir);
        if !path.exists() {
            return Err(eyre!(
                "💥 Template directory specified in system config not found: `{}`.",
                path.display()
            ));
        }
        path
    } else {
        let home_dir = dirs::home_dir().ok_or_else(|| eyre!("💥 Can't find home directory"))?;
        home_dir.join(".boilermaker").join("templates")
    };
    Ok(template_dir)
}

#[tracing::instrument]
pub fn get_template_config_text(template_path: &Path) -> Result<String> {
    let config_path = template_path.join("boilermaker.toml");

    if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        Ok(config_content)
    } else {
        Err(eyre!(
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

pub fn expand_tilde(path: &str) -> Option<PathBuf> {
    if let Some(stripped) = path.strip_prefix("~/") {
        dirs::home_dir().map(|mut home| {
            home.push(stripped);
            home
        })
    } else if path == "~" {
        dirs::home_dir()
    } else {
        Some(PathBuf::from(path))
    }
}
