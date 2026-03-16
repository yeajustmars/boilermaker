use clap::Subcommand;

pub mod get;
pub use get::{Get, get};

#[derive(Subcommand)]
pub enum Config {
    #[command(about = "Get config")]
    Get(Get),
    //#[command(about = "Set config")]
    //Set(Set),
}
