use std::path::PathBuf;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::{Table, settings::Style};
use tracing::info;

use crate::db::template_cache::DisplayableTemplateListResult;
use crate::state::AppState;
use crate::template::remove_dir_if_exists;

#[derive(Parser)]
pub struct Remove {
    #[arg(required = true)]
    pub id: i64,
}

pub async fn remove(app_state: &AppState, cmd: &Remove) -> Result<()> {
    info!("Removing template: {}", cmd.id);

    let cache = app_state
        .template_db
        .write()
        .map_err(|e| eyre!("💥 Failed to acquire write lock: {}", e))?;

    let template = match cache.get_template(cmd.id).await? {
        Some(template) => template,
        None => {
            return Err(eyre!("💥 No template found with ID: {}", cmd.id));
        }
    };

    let template_dir = PathBuf::from(&template.template_dir);

    let rows = vec![template]
        .into_iter()
        .map(DisplayableTemplateListResult::to_std_row)
        .collect::<Vec<_>>();

    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n{table}\n\n");

    if let Err(err) = remove_dir_if_exists(&template_dir) {
        return Err(eyre!("💥 Failed to remove template directory: {}", err));
    }

    let removed_id = cache.delete_template(cmd.id).await?;

    info!("Removed template: {}", removed_id);

    Ok(())
}
