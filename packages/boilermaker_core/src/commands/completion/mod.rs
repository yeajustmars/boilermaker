use clap::Subcommand;

pub mod generate;
pub use generate::{Generate, generate};

#[derive(Subcommand)]
pub enum Completion {
    #[command(about = "Generate autocompletion for Boilermaker")]
    Generate(Generate),
}
