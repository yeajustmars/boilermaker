use clap::{Parser, ValueEnum};
use color_eyre::{Result, eyre::eyre};
use tabled::Tabled;

use crate::{
    db::{DocRow, DocumentId},
    docs::{build_docs_tree, print_docs_tree},
    state::AppState,
    util::{output::print_table, string::truncate_to_char_count},
};

#[derive(Debug, Clone, ValueEnum)]
pub enum ListFormat {
    #[clap(name = "flat")]
    Flat,
    #[clap(name = "tabled")]
    Table,
    #[clap(name = "tree")]
    Tree,
}

#[derive(Debug, Parser)]
pub struct List {
    #[arg(
        short = 'f',
        long,
        help = "Format (flat, table, tree)",
        default_value = "default"
    )]
    pub format: String,
    #[arg(
        short = 'l',
        long,
        default_value_t = false,
        help = "Sugar for `-f flat"
    )]
    pub flat: bool,
    #[arg(
        short = 't',
        long,
        default_value_t = false,
        help = "Sugar for `-f table"
    )]
    pub table: bool,
    #[arg(
        short = 'r',
        long,
        default_value_t = false,
        help = "Sugar for `-f tree"
    )]
    pub tree: bool,
}

#[tracing::instrument]
pub async fn list(app_state: &AppState, cmd: &List) -> Result<()> {
    error_if_more_than_one_format_option_set(cmd)?;

    let docs = app_state.local_db.get_docs().await?;
    let format = get_format(cmd);

    match format {
        "flat" => {
            for doc in docs {
                println!("{}: {}", doc.id, doc.rel_path);
            }
        }
        "tree" => {
            let docs_tree = build_docs_tree(docs);
            print_docs_tree(&docs_tree, 0);
        }
        _ => {
            let rows = docs.into_iter().map(ListTableRow::from).collect::<Vec<_>>();
            print_table(&rows);
        }
    }

    Ok(())
}

#[tracing::instrument]
fn error_if_more_than_one_format_option_set(cmd: &List) -> Result<()> {
    let is_format_set = !cmd.format.eq("default");
    let format_options = vec![cmd.flat, cmd.table, cmd.tree, is_format_set]
        .into_iter()
        .filter(|&b| b)
        .collect::<Vec<bool>>();

    if format_options.len() > 1 {
        return Err(eyre!(
            "Cannot provide more than 1 format option (sugar or otherwise)"
        ));
    }

    Ok(())
}

#[tracing::instrument]
fn get_format(cmd: &List) -> &str {
    if !cmd.format.eq("default") {
        &cmd.format
    } else if cmd.flat {
        "flat"
    } else if cmd.table {
        "table"
    } else if cmd.tree {
        "tree"
    } else {
        "default"
    }
}

#[derive(Debug, Tabled)]
pub struct ListTableRow {
    pub id: DocumentId,
    pub rel_path: String,
    pub title: String,
}

impl From<DocRow> for ListTableRow {
    fn from(doc: DocRow) -> Self {
        let title = if let Some(title) = &doc.title {
            if title.chars().count() > 25 {
                let mut s = truncate_to_char_count(title, 25);
                s.push_str("...");
                s
            } else {
                title.to_string()
            }
        } else {
            "".to_string()
        };

        Self {
            id: doc.id,
            rel_path: doc.rel_path,
            title,
        }
    }
}
