use clap::Subcommand;

pub mod list;

pub use list::{List, list};

#[derive(Subcommand)]
pub enum Docs {
    #[command(about = "List sources")]
    List(List),
}
