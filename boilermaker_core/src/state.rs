use std::fmt;
use std::sync::Arc;

use crate::config::SysConfig;
use crate::db::TemplateDb;

pub type TemplateDbType = Arc<dyn TemplateDb + Send + Sync>;

pub struct AppState {
    pub log_level: u8,
    pub sys_config: SysConfig,
    pub local_db: TemplateDbType,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppState {{ cache_db, source_db, sys_config, ... }}")
    }
}
