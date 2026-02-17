use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use reqwest::{StatusCode, get as http_get};
use serde::Deserialize;
use tracing::info;

use crate::{
    db::source::{PartialSourceTemplateRow, SourceRow},
    state::AppState,
    template::{
        CloneContext, clean_dir, clone_repo, get_lang, get_template_config_text,
        make_name_from_url, make_tmp_dir_from_url, template_config_text_to_config,
    },
};

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(required = true, help = "Source URL or file path")]
    coordinate: String,
}

#[tracing::instrument]
pub async fn add(app_state: &AppState, cmd: &Add) -> Result<()> {
    let coordinate = cmd.coordinate.trim().to_owned();
    let base_url = coordinate.replace("/boilermaker_source.toml", "");

    let src_text = http_get(&coordinate).await?.text().await?;
    let src_cnf: SourceConfig = toml::from_str(&src_text)?;

    let readme_url = format!("{base_url}/README.md");
    let readme_response = http_get(&readme_url).await?;
    let readme = match readme_response.status() {
        StatusCode::OK => Some(readme_response.text().await?),
        _ => None,
    };

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
        readme,
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

        let base_work_dir = if let Some(subdir) = &template.get("subdir") {
            clone_dir.join(subdir)
        } else {
            clone_dir.to_path_buf()
        };
        let base_path = base_work_dir.as_path();
        let cnf_text = get_template_config_text(base_path)?;
        let cnf = template_config_text_to_config(&cnf_text)?;
        let lang = get_lang(&cnf, &template.get("lang").cloned())?;
        let work_dir = base_work_dir.join(&lang);

        let partial_row = PartialSourceTemplateRow {
            name: name.clone(),
            lang: lang.clone(),
            repo: repo.to_owned(),
            config: cnf_text,
            branch: template.get("branch").cloned(),
            subdir: template.get("subdir").cloned(),
        };

        partial_source_template_rows.push((work_dir, partial_row));
    }

    let sources = app_state.local_db.clone();
    let r = sources
        .add_source(source_row, partial_source_template_rows)
        .await?;
    info!("Source added with Source ID: {}", r.source_id);
    info!("Added {} Templates to Source", r.source_template_ids.len());

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
