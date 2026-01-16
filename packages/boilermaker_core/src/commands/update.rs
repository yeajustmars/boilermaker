use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use color_eyre::eyre::eyre;
use tracing::info;

use crate::db::TemplateRow;
use crate::state::AppState;
use crate::template::{
    CloneContext, clean_dir, clone_repo, install_template, make_tmp_dir_from_url,
};
use crate::util::file::remove_git_dir;

#[derive(Debug, Parser)]
pub struct Update {
    #[arg(required = true)]
    pub id: i32,
}

#[tracing::instrument]
pub async fn update(app_state: &AppState, cmd: &Update) -> Result<()> {
    let cache = app_state.local_db.clone();
    let Some(templ) = cache.get_template(cmd.id as i64).await? else {
        return Err(eyre!("💥 Cannot find template: {}.", cmd.id));
    };

    info!("Updating template #{}: {}", templ.id, templ.name);

    // TODO: fix template_dir to be new hash based path
    let template_dir = PathBuf::from(templ.template_dir.clone());
    let tmp_clone_dir = make_tmp_dir_from_url(&templ.repo);

    let clone_ctx = CloneContext::new(
        &templ.repo,
        Some(tmp_clone_dir.clone()),
        templ.branch.clone(),
    );
    clone_repo(&clone_ctx).await?;
    clean_dir(&template_dir)?;
    install_template(&tmp_clone_dir, &template_dir).await?;
    remove_git_dir(&template_dir)?;

    let row = TemplateRow::from(templ.clone());
    cache.update_template(templ.id, row).await?;

    info!("✅ Template updated!");
    Ok(())
}
