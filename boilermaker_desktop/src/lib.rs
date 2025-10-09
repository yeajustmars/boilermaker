use std::sync::{Arc, RwLock};

use color_eyre::eyre::{eyre, Result};
use once_cell::sync::OnceCell;

use boilermaker_core::{
    config::{get_system_config, DEFAULT_LOCAL_CACHE_PATH_STRING},
    db::{LocalCache, TemplateDb},
    state::AppState,
};

pub type TemplateDbType = Arc<RwLock<dyn TemplateDb + Send + Sync>>;

pub static APP_STATE: OnceCell<AppState> = OnceCell::new();

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

            if !template_db
                .read()
                .unwrap()
                .template_table_exists()
                .await
                .unwrap_or(false)
            {
                template_db
                    .write()
                    .unwrap()
                    .create_template_table()
                    .await
                    .map_err(|e| eyre!("Failed to create template table: {}", e))?;
            }

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
