use std::collections::HashMap;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::{Result, eyre::eyre};
use serde::Deserialize;
use tabled::{Table, Tabled, settings::Style};
use tracing::info;

use crate::db::TabledSourceRow;
use crate::db::source::{PartialSourceTemplateRow, SourceRow};
use crate::state::AppState;
use crate::template::{
    CloneContext, clean_dir, clone_repo, get_lang, get_template_config, make_name_from_url,
    make_tmp_dir_from_url,
};
use crate::util::string;

#[derive(Subcommand)]
pub enum Sources {
    #[command(about = "Add a source")]
    Add(Add),
    #[command(about = "List added sources")]
    List(List),
}

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(required = true, help = "Source URL or file path")]
    coordinate: String,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub source: HashMap<String, String>,
    pub templates: Vec<HashMap<String, String>>,
}

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

#[derive(Debug, Parser)]
pub struct List {
    #[arg(short = 'l', long, help = "List only local sources")]
    pub local: bool,
}

#[derive(Debug, Deserialize, Tabled)]
pub struct SourceMap {
    pub name: String,
    pub backend: String,
    pub description: String,
}

impl From<&HashMap<String, String>> for SourceMap {
    #[tracing::instrument]
    fn from(m: &HashMap<String, String>) -> Self {
        let description = m.get("description").cloned().unwrap_or_default();
        let description = if description.len() > 50 {
            string::truncate_to_char_count(&description, 50) + "..."
        } else {
            description
        };

        SourceMap {
            name: m.get("name").cloned().unwrap_or_default(),
            backend: m.get("backend").cloned().unwrap_or_default(),
            description,
        }
    }
}

#[tracing::instrument]
pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    let sources = app_state.local_db.list_sources().await?;
    if sources.is_empty() {
        info!("No sources found.");
        info!("💡 Have a look at `boil sources add`");
        return Ok(());
    }

    let table_rows = sources
        .into_iter()
        .map(TabledSourceRow::from)
        .collect::<Vec<_>>();
    let mut table = Table::new(&table_rows);
    table.with(Style::psql());
    print!("\n\n{table}\n\n");

    Ok(())
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
