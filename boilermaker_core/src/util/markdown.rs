use std::path::PathBuf;

use color_eyre::Result;
use pulldown_cmark::{Parser, html::push_html};

use crate::util::file;

#[tracing::instrument]
pub fn md_file_to_html(path: &PathBuf) -> Result<String> {
    let s = file::read_file_to_string(path)?;
    let parser = Parser::new(&s);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    Ok(html_output)
}

#[tracing::instrument]
pub fn md_str_to_html(s: &str) -> Result<String> {
    let parser = Parser::new(s);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    Ok(html_output)
}
