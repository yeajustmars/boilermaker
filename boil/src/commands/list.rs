use clap::Parser;
use color_eyre::Result;
use tracing::info;

use crate::AppState;

#[derive(Parser)]
pub struct List {
    #[arg(short = 'u', long)]
    pub public: bool,
    #[arg(short = 'p', long)]
    pub private: bool,
}

pub async fn list(_sys_config: &AppState, _cmd: &List) -> Result<()> {
    info!("Listing templates...");
    /*
    let db_path = BOILERMAKER_LOCAL_CACHE_PATH
        .to_str()
        .ok_or_else(|| eyre!("Failed to convert path to string"))?;
    let cache = LocalCache::new(db_path).await?;
    let templates = cache.get_templates().await?;

    for (i, template) in templates.iter().enumerate() {
        println!("{}: {} ({})", i + 1, template.name, template.lang);
    }
    */

    Ok(())
}
