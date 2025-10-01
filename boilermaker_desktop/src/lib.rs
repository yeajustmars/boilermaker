use std::fmt;
use std::sync::{Arc, RwLock};

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::OnceCell;
use toml;

use boilermaker_core::config::{get_system_config, DEFAULT_LOCAL_CACHE_PATH_STRING};
use boilermaker_core::db::{LocalCache, TemplateDb};

pub type TemplateDbType = Arc<RwLock<dyn TemplateDb + Send + Sync>>;

pub static APP_STATE: OnceCell<AppState> = OnceCell::new();

#[derive(Clone)]
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

/// Initialize the global application state into a OnceCell APP_STATE
pub fn init_app_state() -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let template_db = Arc::new(RwLock::new(
                LocalCache::new(DEFAULT_LOCAL_CACHE_PATH_STRING.as_str())
                    .await
                    .expect("Failed to initialize local cache"),
            ));
            let sys_config = get_system_config(None).expect("Failed to load system config");
            let app_state = AppState {
                template_db,
                sys_config,
                log_level: 1,
            };

            APP_STATE
                .set(app_state)
                .map_err(|_| eyre!("Failed to set APP_STATE"))?;
            Ok::<(), color_eyre::eyre::Report>(())
        })?;
    Ok(())
}
