use std::{
    collections::HashMap,
    fs,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Error, Result, eyre};
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use toml::{Value, map::Map as TomlMap};
use tracing::{info, warn};

lazy_static! {
    pub static ref SYS_CONFIG_FILE: String = format!(
        "{}/.config/boilermaker/boilermaker.toml",
        home_dir().unwrap().to_str().unwrap()
    );
    pub static ref DEFAULT_LOCAL_CACHE_PATH: PathBuf = make_boilermaker_local_cache_path().unwrap();
    pub static ref DEFAULT_LOCAL_CACHE_PATH_STRING: String = DEFAULT_LOCAL_CACHE_PATH
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
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
pub fn make_boilermaker_local_cache_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Can't find home directory"))?;
    let local_cache_dir = home_dir.join(".boilermaker");

    fs::create_dir_all(local_cache_dir)?;

    let local_cache_path = home_dir.join(".boilermaker").join("local_cache.db");

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&local_cache_path)
    {
        Ok(_) => Ok(local_cache_path),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return Ok(local_cache_path);
            } else {
                return Err(eyre!("💥 Failed to create local cache file: {}", e));
            }
        }
    }
}

#[tracing::instrument]
pub fn get_template_config(template_path: &Path) -> Result<TemplateConfig> {
    let config_path = template_path.join("boilermaker.toml");

    if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        let config: TemplateConfig = toml::from_str(&config_content)?;
        Ok(config)
    } else {
        Err(color_eyre::eyre::eyre!(
            "❗ Config file not found at `{}`.",
            config_path.display()
        ))
    }
}

// TODO: decide on whether variables are allowed to be nested or not.
// TODO: decide on whether variables should allow aggregate types (arrays, tables) or just simple key-value pairs.
// NOTE: Probably yes to the latter.
#[derive(Debug, Deserialize)]
pub struct TemplateConfig {
    pub project: TemplateConfigProject,
    // pub variables: Option<toml::Value>,
    //pub variables: Option<HashMap<String, String>>,
    pub variables: Option<TemplateConfigVariableMap>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateConfigProject {
    // pub name: String,
    // pub repository: String,
    // pub subdir: Option<String>,
    // pub version: Option<String>,
    pub default_lang: Option<String>,
    // pub description: Option<String>,
    // pub authors: Option<Vec<String>>,
    // pub license: Option<String>,
    // pub keywords: Option<Vec<String>>,
    // pub website: Option<String>,
}

#[derive(Debug)]
pub struct TemplateConfigVariableMap(HashMap<String, String>);

impl TemplateConfigVariableMap {
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.0
    }
}

impl<'de> Deserialize<'de> for TemplateConfigVariableMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(TemplateConfigVariableMapVisitor)
    }
}

struct TemplateConfigVariableMapVisitor;

impl<'de> Visitor<'de> for TemplateConfigVariableMapVisitor {
    type Value = TemplateConfigVariableMap;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a TOML table that can be converted to a HashMap<String, String>")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut m = HashMap::new();
        while let Some(key) = map.next_key::<String>()? {
            let value: toml::Value = map.next_value()?;
            let s = match value {
                toml::Value::String(s) => s,
                other_type => other_type.to_string(),
            };
            m.insert(key, s);
        }
        Ok(TemplateConfigVariableMap(m))
    }
}
