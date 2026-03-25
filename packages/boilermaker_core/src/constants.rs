use std::path::PathBuf;

use dirs::home_dir;
use lazy_static::lazy_static;
use regex::Regex;

use crate::config::{get_template_base_dir, make_local_db_path};

pub const BRANCH_REGEX: &str = r"^(refs/heads/)?[A-Za-z0-9._/-]+$";
pub const SUBDIR_REGEX: &str = r"^/?[A-Za-z0-9/\-_].*$";
pub const TEMPLATE_FILEPATH_VAR_REGEX: &str = r"(?<underscore>___.*?___)|(?<dash>---.*?---)";
pub const URL_PREFIX_REGEX: &str = r"^(https?|git|ssh|ftp|ftps)://.*?/";

lazy_static! {
    pub static ref BRANCH_PATTERN: Regex = Regex::new(BRANCH_REGEX).unwrap();
    pub static ref SUBDIR_PATTERN: Regex = Regex::new(SUBDIR_REGEX).unwrap();
    pub static ref DEFAULT_CONFIG_FILE: String = format!(
        "{}/.config/boilermaker/boilermaker.toml",
        home_dir().unwrap().to_str().unwrap()
    );
    pub static ref DEFAULT_ETC_SYS_CONFIG_FILE: String =
        "/etc/boilermaker/boilermaker.toml".to_string();
    pub static ref DEFAULT_LOG_LEVEL: String = "INFO".to_string();
    pub static ref DEFAULT_VAR_LIB_DB_PATH: PathBuf =
        PathBuf::from("/var/lib/boilermaker/boilermaker.db");
    pub static ref DEFAULT_VAR_LIB_DB_PATH_STRING: String = DEFAULT_VAR_LIB_DB_PATH
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
    pub static ref DEFAULT_LOCAL_DB_PATH: PathBuf = make_local_db_path(None).unwrap();
    pub static ref DEFAULT_LOCAL_DB_PATH_STRING: String = DEFAULT_LOCAL_DB_PATH
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
    pub static ref DEFAULT_TEMPLATE_DIR: PathBuf = get_template_base_dir(None).unwrap();
    pub static ref DEFAULT_TEMPLATE_DIR_STRING: String =
        DEFAULT_TEMPLATE_DIR.as_path().to_str().unwrap().to_string();
    pub static ref TEMPLATE_FILEPATH_VAR_PATTERN: Regex =
        Regex::new(TEMPLATE_FILEPATH_VAR_REGEX).unwrap();
    pub static ref URL_PREFIX_PATTERN: Regex = Regex::new(URL_PREFIX_REGEX).unwrap();
}
