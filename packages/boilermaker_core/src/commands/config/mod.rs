use clap::Subcommand;

pub mod get;
pub use get::{Get, get};

pub mod set;
pub use set::{Set, set};

#[derive(Subcommand)]
pub enum Config {
    #[command(about = "Get Boilermaker config")]
    Get(Get),
    #[command(about = "Set Boilermaker config")]
    Set(Set),
}
