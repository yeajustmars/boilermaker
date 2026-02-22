use clap::Subcommand;

pub mod list;
pub use list::{List, list};

pub mod view;
pub use view::{View, view};

#[derive(Subcommand)]
pub enum Docs {
    #[command(about = "List sources")]
    List(List),
    #[command(about = "View a doc")]
    View(View),
}
