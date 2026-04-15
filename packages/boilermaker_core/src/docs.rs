use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

use colored::Colorize;
use rust_embed::RustEmbed;
use serde::Serialize;

use crate::db::DocRow;

// TODO: review if doc files are needed if all are indexed in DB
#[derive(RustEmbed, Clone, Debug)]
#[folder = "./docs/"]
pub struct DocFiles;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DocTreeNode {
    File {
        name: String,
        name_pretty: String,
        filename: String,
        rel_url: String,
        full_path: String,
    },
    Dir {
        name: String,
        children: Vec<DocTreeNode>,
    },
}

impl DocTreeNode {
    #[tracing::instrument]
    pub fn sort_children(&mut self) {
        if let DocTreeNode::Dir { children, .. } = self {
            children.sort_by(|a, b| match (a, b) {
                (DocTreeNode::Dir { .. }, DocTreeNode::File { .. }) => Ordering::Less,
                (DocTreeNode::File { .. }, DocTreeNode::Dir { .. }) => Ordering::Greater,
                (a, b) => a.name().cmp(b.name()),
            });
            for child in children {
                child.sort_children();
            }
        }
    }

    #[tracing::instrument]
    pub fn name(&self) -> &str {
        match self {
            DocTreeNode::File { name, .. } => name,
            DocTreeNode::Dir { name, .. } => name,
        }
    }
}

#[tracing::instrument]
pub fn build_docs_tree(docs: Vec<DocRow>) -> Vec<DocTreeNode> {
    let mut root_nodes: Vec<DocTreeNode> = Vec::new();

    for doc in docs {
        let path = Path::new(&doc.rel_path);
        let parts: Vec<&str> = path.iter().map(|os_str| os_str.to_str().unwrap()).collect();

        insert_doc_recursive(&mut root_nodes, &parts, doc.rel_path.clone());
    }

    root_nodes.sort_by(|a, b| a.name().cmp(b.name()));

    root_nodes
}

#[tracing::instrument]
fn insert_doc_recursive(nodes: &mut Vec<DocTreeNode>, parts: &[&str], full_path: String) {
    if let Some((current_part, rest_parts)) = parts.split_first() {
        if rest_parts.is_empty() {
            let name = Path::new(current_part)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let name_pretty = name
                .split(|c| c == '-' || c == '_')
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");

            nodes.push(DocTreeNode::File {
                name,
                name_pretty,
                rel_url: rel_url(&full_path),
                filename: current_part.to_string(),
                full_path,
            });
        } else {
            let mut dir_index = None;
            for (i, node) in nodes.iter().enumerate() {
                if let DocTreeNode::Dir { name, .. } = node
                    && name == *current_part
                {
                    dir_index = Some(i);
                    break;
                }
            }

            if dir_index.is_none() {
                nodes.push(DocTreeNode::Dir {
                    name: current_part.to_string(),
                    children: Vec::new(),
                });
                dir_index = Some(nodes.len() - 1);
            }

            if let Some(idx) = dir_index
                && let DocTreeNode::Dir { children, .. } = &mut nodes[idx]
            {
                insert_doc_recursive(children, rest_parts, full_path);
            }
        }
    }
}

#[tracing::instrument]
pub fn print_docs_tree(nodes: &[DocTreeNode], depth: usize) {
    let indent = "    ".repeat(depth);
    for node in nodes {
        match node {
            DocTreeNode::File { name, .. } => {
                println!("{}{}", indent, name);
            }
            DocTreeNode::Dir { name, children } => {
                println!("{}{}/", indent, name.bold().blue());
                print_docs_tree(children, depth + 1);
            }
        }
    }
}

#[tracing::instrument]
pub fn rel_url(rel_path: &str) -> String {
    let path = PathBuf::from(rel_path)
        .with_extension("")
        .to_str()
        .unwrap()
        .to_string();
    format!("/docs/{}", path)
}
