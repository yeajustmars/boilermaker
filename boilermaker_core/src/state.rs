use std::fmt;
use std::sync::{Arc, RwLock};

use crate::db::TemplateDb;

pub type TemplateDbType = Arc<RwLock<dyn TemplateDb + Send + Sync>>;

pub struct AppState {
    pub template_db: TemplateDbType,
    pub sys_config: toml::Value,
    pub log_level: u8,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppState {{ template_db: ... }}")
    }
}
