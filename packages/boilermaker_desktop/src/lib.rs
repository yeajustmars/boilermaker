use std::sync::Arc;

use tokio::sync::RwLock;

use boilermaker_core::db::TemplateDb;

pub type TemplateDbType = Arc<RwLock<dyn TemplateDb + Send + Sync>>;

mod app_state;
pub use app_state::{init_app_state, APP_STATE};

mod templates_context;
pub use templates_context::{init_templates_context, TemplatesContext};
