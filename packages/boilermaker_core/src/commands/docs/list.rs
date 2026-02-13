use clap::{Parser, ValueEnum};
use color_eyre::{Result, eyre::eyre};
use tabled::Tabled;

use crate::{
    db::{DocRow, DocumentId},
    docs::{build_docs_tree, print_docs_tree},
    state::AppState,
    util::{output::print_table, string::truncate_to_char_count, time::timestamp_to_iso8601},
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
        default_value = "table"
    )]
    pub format: String,
}

#[tracing::instrument]
pub async fn list(app_state: &AppState, cmd: &List) -> Result<()> {
    let docs = app_state.local_db.get_docs().await?;
    let format = cmd.format.as_str();

    match format {
        "flat" => {
            for doc in docs {
                println!("{}: {}", doc.id, doc.rel_path);
            }
        }
        "table" => {
            let rows = docs.into_iter().map(ListTableRow::from).collect::<Vec<_>>();
            print_table(&rows);
        }
        "tree" => {
            let docs_tree = build_docs_tree(docs);
            print_docs_tree(&docs_tree, 0);
        }
        _ => {
            return Err(eyre!("Invalid format: {}", format));
        }
    }

    Ok(())
}

#[derive(Debug, Tabled)]
pub struct ListTableRow {
    pub id: DocumentId,
    pub rel_path: String,
    pub created_at: String,
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
            created_at: timestamp_to_iso8601(doc.created_at as i64),
            title,
        }
    }
}
