use std::sync::Arc;

use color_eyre::eyre::{Result, eyre};
use once_cell::sync::OnceCell;

use boilermaker_core::{
    config::{DEFAULT_LOCAL_DB_PATH_STRING, get_system_config},
    db::{LocalDb, TemplateDb, TemplateMethods},
    state::AppState,
};

pub static APP_STATE: OnceCell<AppState> = OnceCell::new();

/// Initialize the global application state into a OnceCell APP_STATE
pub fn init_app_state() -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let db_path = DEFAULT_LOCAL_DB_PATH_STRING.as_str();
            let db = Arc::new(LocalDb::new(db_path).await.map_err(|err| {
                eyre!(
                    "Failed to initialize local db at path '{}': {}",
                    db_path,
                    err
                )
            })?);
            if !db.template_table_exists().await.unwrap_or(false) {
                db.create_schema()
                    .await
                    .map_err(|e| eyre!("Failed to initialize local db: {}", e))?;
            }
            // App state
            let sys_config = get_system_config(None).expect("Failed to load system config");
            let app_state = AppState {
                local_db: db,
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
