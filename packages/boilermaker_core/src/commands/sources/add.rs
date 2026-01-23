use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use serde::Deserialize;
use tracing::info;

use crate::db::source::{PartialSourceTemplateRow, SourceRow};
use crate::state::AppState;
use crate::template::{
    CloneContext, clean_dir, clone_repo, get_lang, get_template_config, make_name_from_url,
    make_tmp_dir_from_url,
};

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(required = true, help = "Source URL or file path")]
    coordinate: String,
}

#[tracing::instrument]
pub async fn add(app_state: &AppState, cmd: &Add) -> Result<()> {
    let coordinate = cmd.coordinate.trim().to_owned();
    let src_text = reqwest::get(&coordinate).await?.text().await?;
    let src_cnf: SourceConfig = toml::from_str(&src_text)?;

    let name = match src_cnf.source.get("name") {
        Some(source_name) => source_name.to_string(),
        None => return Err(eyre!("Template missing 'source_name' field")),
    };

    let backend = match src_cnf.source.get("backend") {
        Some(backend) => backend.to_string(),
        None => return Err(eyre!("Template missing 'backend' field")),
    };

    let description = src_cnf.source.get("description");

    let source_row = SourceRow {
        name,
        backend,
        description: description.cloned(),
        coordinate: coordinate.to_owned(),
        sha256_hash: None,
    };
    let source_row = source_row.set_hash_string();

    let mut partial_source_template_rows: Vec<(PathBuf, PartialSourceTemplateRow)> = Vec::new();
    for template in src_cnf.templates.iter() {
        let repo = match template.get("repo") {
            Some(repo) => repo,
            None => return Err(eyre!("Template missing 'repo' field")),
        };

        let name = if let Some(name) = &template.get("name") {
            name.to_string()
        } else {
            make_name_from_url(repo)
        };

        let repo_ctx = CloneContext::from(template);
        let clone_dir = repo_ctx.dest.as_ref().unwrap();

        if let Err(err) = clean_dir(clone_dir) {
            return Err(eyre!("💥 Failed setting up clone dir: {}", err));
        }

        info!("Cloning source template: {name}");
        if let Err(err) = clone_repo(&repo_ctx).await {
            return Err(eyre!("💥 Failed to clone template: {}", err));
        }

        let work_dir = if let Some(subdir) = &template.get("subdir") {
            clone_dir.join(subdir)
        } else {
            clone_dir.to_path_buf()
        };

        let cnf = get_template_config(work_dir.as_path())?;
        let lang = get_lang(&cnf, &template.get("lang").cloned())?;

        let partial_row = PartialSourceTemplateRow {
            name: name.clone(),
            lang: lang.clone(),
            repo: repo.to_owned(),
            branch: template.get("branch").cloned(),
            subdir: template.get("subdir").cloned(),
        };

        partial_source_template_rows.push((work_dir, partial_row));
    }

    let sources = app_state.local_db.clone();
    let r = sources
        .add_source(source_row, partial_source_template_rows)
        .await?;
    info!("Source added with ID: {r:#?}");

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub source: HashMap<String, String>,
    pub templates: Vec<HashMap<String, String>>,
}

impl From<&HashMap<String, String>> for CloneContext {
    #[tracing::instrument]
    fn from(m: &HashMap<String, String>) -> Self {
        let repo = m.get("repo").cloned().unwrap();
        Self {
            url: repo.clone(),
            branch: m.get("branch").cloned(),
            dest: Some(make_tmp_dir_from_url(&repo)),
        }
    }
}
