use std::collections::HashMap;

use clap::{Parser, Subcommand};
use color_eyre::eyre;
use color_eyre::{Result, eyre::eyre};
use serde::Deserialize;
use tabled::{Table, Tabled, settings::Style};
use tracing::info;

//use crate::db::local_db::{SourceRow, SourceTemplateRow, hashmap_into_source_template_row};
use crate::state::AppState;
use crate::template::{
    CloneContext, clean_dir, clone_repo, get_template_config, make_name_from_url,
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

pub async fn add(_app_state: &AppState, cmd: &Add) -> Result<()> {
    let coordinate = cmd.coordinate.trim().to_owned();
    let src_text = reqwest::get(&coordinate).await?.text().await?;
    let cnf: SourceConfig = toml::from_str(&src_text)?;
    println!("Source Config: {cnf:?}");

    // TODO: 1. [ ] Validate that each repo/lang is a real source
    //       NOTE: (just clone the repo as we'll need it on success anyway)
    // TODO: 2. [ ] Create source entry in DB
    // TODO: 3. [ ] Create template entries in DB

    for template in cnf.templates.iter() {
        let repo = match template.get("repo") {
            Some(repo) => repo,
            None => return Err(eyre!("Template missing 'repo' field")),
        };

        let name = if let Some(name) = &template.get("name") {
            name.to_string()
        } else {
            make_name_from_url(repo)
        };

        // TODO: consolidate with same clone logic in install.rs
        let repo_ctx = CloneContext::from(template);
        let clone_dir = repo_ctx.dest.as_ref().unwrap();

        if let Err(err) = clean_dir(clone_dir) {
            return Err(eyre!("💥 Failed setting up clone dir: {}", err));
        }

        info!("Cloning template");
        if let Err(err) = clone_repo(&repo_ctx).await {
            return Err(eyre!("💥 Failed to clone template: {}", err));
        }

        let work_dir = if let Some(subdir) = &template.get("subdir") {
            clone_dir.join(subdir)
        } else {
            clone_dir.to_path_buf()
        };

        // let cnf = get_template_config(work_dir.as_path())?;
        // let lang = get_lang(&cnf, &template.lang)?;
        // let template_dir = get_template_dir_path(&name)?;

        println!("\n\\==========================================================");
        println!("Cloning template repo: {repo_ctx:?}");
        println!("\tname: {name:?}");
        println!("\tclone_dir: {clone_dir:?}");
        println!("\twork_dir: {work_dir:?}");
        println!("/==========================================================\n");
    }

    /*
    for template in cnf.templates.iter() {
        let row = hashmap_into_source_template_row(template, &coordinate)?;
        println!("Template Row: {:?}", row);
    }
     */

    /*
    let name = cnf.source.get("name").cloned().unwrap();

    let source_row = SourceRow {
        name: name.clone(),
        backend: cnf.source.get("backend").cloned().unwrap(),
        coordinate: coordinate.clone(),
        sha256_hash: None,
    };
    let source_row = source_row.set_hash_string();

    let sources = app_state.local_db.clone();

    let source_id = match sources.create_source(source_row).await {
        Ok(id) => {
            info!("Source '{}' added successfully.", name);
            id
        }
        Err(e) => {
            return Err(eyre!("💥 Failed to add source: {}", e));
        }
    };
     */

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
pub fn print_sources_table(sources: Vec<HashMap<String, String>>) -> Result<()> {
    let rows = sources.iter().map(SourceMap::from).collect::<Vec<_>>();

    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n\n{table}\n\n");

    Ok(())
}

// TODO: add filters/options
#[tracing::instrument]
pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    if let Some(sources) = &app_state.sys_config.sources {
        print_sources_table(sources.to_vec())?;
        Ok(())
    } else {
        info!("No sources found.");
        info!("💡 Have a look at `boil sources add`");
        Ok(())
    }
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
