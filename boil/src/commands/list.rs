use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::info;

use crate::local_cache::{BOILERMAKER_LOCAL_CACHE_PATH, LocalCache};

#[derive(Parser)]
pub(crate) struct List {
    #[arg(short = 'u', long)]
    pub public: bool,
    #[arg(short = 'p', long)]
    pub private: bool,
}

pub(crate) async fn list(_sys_config: &toml::Value, _cmd: &List) -> Result<()> {
    info!("Listing templates...");
    let db_path = BOILERMAKER_LOCAL_CACHE_PATH
        .to_str()
        .ok_or_else(|| eyre!("Failed to convert path to string"))?;

    let cache = LocalCache::new(db_path).await?;

    let templates = cache.get_templates().await?;

    /*
    let templates: Vec<TemplateResult> = if cmd.public {
        cache.list_public_templates().await?
    } else if cmd.private {
        cache.list_private_templates().await?
    } else {
        cache.list_all_templates().await?
    };
     */

    for template in templates {
        println!(
            "- {} (lang: {}, dir: {}, created_at: {}, updated_at: {}, repo: {}, branch: {}, subdir: {})",
            template.name,
            template.lang,
            template.created_at,
            template.updated_at,
            template.template_dir,
            template.repo,
            template.branch.as_deref().unwrap_or("default"),
            template.subdir.as_deref().unwrap_or("root"),
        );
    }

    Ok(())
}
