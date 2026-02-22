use tabled::Tabled;

use crate::{
    db::{DocRow, SourceResult, SourceTemplateResult, TemplateResult, doc::DocumentId},
    util::output::print_table_error,
};

#[derive(Tabled)]
struct MultipleTemplateResultsRow {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Lang")]
    lang: String,
    #[tabled(rename = "Repo")]
    repo: String,
    #[tabled(rename = "Subdir")]
    subdir: String,
}

#[tracing::instrument]
pub fn print_multiple_template_results_help(template_rows: &Vec<TemplateResult>) {
    let help_line = "Multiple templates found matching name. Use ID instead.";
    let mut help_rows = Vec::new();
    for t in template_rows {
        help_rows.push(MultipleTemplateResultsRow {
            id: t.id,
            name: t.name.clone(),
            lang: t.lang.clone(),
            repo: t.repo.clone(),
            subdir: t.subdir.clone().unwrap(),
        });
    }

    print_table_error(&help_rows, Some(help_line));
}

#[derive(Tabled)]
struct MultipleSourceResultsRow {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Coordinate")]
    coordinate: String,
    #[tabled(rename = "SHA256 Hash")]
    sha256_hash: String,
}

#[tracing::instrument]
pub fn print_multiple_source_results_help(source_rows: &Vec<SourceResult>) {
    let help_line = "Multiple sources found matching name. Use ID instead.";
    let mut help_rows = Vec::new();
    for s in source_rows {
        help_rows.push(MultipleSourceResultsRow {
            id: s.id,
            name: s.name.clone(),
            coordinate: s.coordinate.clone(),
            sha256_hash: s.sha256_hash.clone().unwrap(),
        });
    }

    print_table_error(&help_rows, Some(help_line));
}

#[tracing::instrument]
pub fn print_multiple_source_template_results_help(template_rows: &Vec<SourceTemplateResult>) {
    let help_line = "Multiple source templates found matching name. Use ID instead.";
    let mut help_rows = Vec::new();
    for t in template_rows {
        help_rows.push(MultipleTemplateResultsRow {
            id: t.id,
            name: t.name.clone(),
            lang: t.lang.clone(),
            repo: t.repo.clone(),
            subdir: t.subdir.clone().unwrap(),
        });
    }

    print_table_error(&help_rows, Some(help_line));
}

#[tracing::instrument]
pub fn print_multiple_doc_results_help(doc_rows: &Vec<DocRow>) {
    let help_line = "Multiple docs found matching name. Use ID instead.";
    let mut help_rows = Vec::new();
    for d in doc_rows {
        help_rows.push(MultipleDocResultsRow {
            id: d.id,
            rel_path: d.rel_path.clone(),
            title: d.title.clone().unwrap_or("".to_string()),
        });
    }

    print_table_error(&help_rows, Some(help_line));
}

#[derive(Tabled)]
pub struct MultipleDocResultsRow {
    pub id: DocumentId,
    pub rel_path: String,
    pub title: String,
}
