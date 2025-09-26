use std::fmt;

use db::TemplateDb;
use std::sync::{Arc, RwLock};

pub type TemplateDbType = Arc<RwLock<dyn TemplateDb + Send + Sync>>;

pub struct AppState {
    pub template_db: TemplateDbType,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppState {{ template_db: ... }}")
    }
}
