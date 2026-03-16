use clap::Subcommand;

pub mod add;
pub mod list;
pub mod show;
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
    #[command(about = "Show source details")]
    Show(show::Show),
    #[command(subcommand, about = "Manage source templates")]
    Templates(Templates),
}
