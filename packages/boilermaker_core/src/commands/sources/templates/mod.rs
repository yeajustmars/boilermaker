use clap::Subcommand;

pub mod install;
pub mod list;
pub mod show;

pub use install::{Install, install};
pub use list::{List, list};

#[derive(Subcommand)]
pub enum Templates {
    #[command(about = "Install from source")]
    Install(Install),
    #[command(about = "List templates for source")]
    List(List),
    #[command(about = "Show source template details")]
    Show(show::Show),
}
