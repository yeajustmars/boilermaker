use clap::Subcommand;

pub mod add;
pub mod list;
pub mod templates;

pub use add::{Add, add};
pub use list::{List, list};
use templates::Templates;

#[derive(Subcommand)]
pub enum Sources {
    #[command(about = "Add a source")]
    Add(Add),
    #[command(about = "List sources")]
    List(List),
    #[command(subcommand, about = "Manage source templates")]
    Templates(Templates),
}
