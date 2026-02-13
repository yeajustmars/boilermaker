use std::sync::Arc;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use termimad::MadSkin;

use crate::{
    db::{DocFindParams, DocRow, TemplateDb},
    state::AppState,
    util::help,
};

#[derive(Debug, Parser)]
pub struct View {
    #[arg(required = true)]
    pub id_or_name: String,
    #[arg(short, long, default_value_t = false, help = "Print raw Markdown")]
    pub raw: bool,
}

#[tracing::instrument]
pub async fn view(app_state: &AppState, cmd: &View) -> Result<()> {
    let doc = {
        let cache = app_state.local_db.clone();
        match cmd.id_or_name.parse::<i64>() {
            Ok(id) => cache.get_doc(id).await?,
            Err(_) => get_existing_docs(cache, &cmd.id_or_name).await?,
        }
    };

    // TODO: detect light/dark mode in OS for Markdown printing
    let skin = MadSkin::default();
    eprintln!("\n{}", skin.term_text(&doc.content));

    Ok(())
}

async fn get_existing_docs(cache: Arc<dyn TemplateDb>, id_or_name: &str) -> Result<DocRow> {
    let mut name = id_or_name.to_string();
    if !name.ends_with(".md") {
        name.push_str(".md");
    }

    let query = DocFindParams {
        rel_path: Some(name.clone()),
        id: None,
        content: None,
        title: None,
        created_at: None,
    };

    let existing_docs = cache.find_docs(query).await?;

    match existing_docs.len() {
        0 => Err(eyre!("💥 Cannot find doc: {}.", name))?,
        1 => Ok(existing_docs[0].to_owned()),
        2.. => {
            help::print_multiple_doc_results_help(&existing_docs);
            Err(eyre!(
                "💥 Found multiple results matching template: {}.",
                name
            ))?
        }
    }
}
