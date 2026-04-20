use clap::Subcommand;

pub mod generate;
pub use generate::{GenBash, gen_bash};

#[derive(Subcommand)]
pub enum Completion {
    #[command(about = "Get BASH autocomplete for Boilermaker")]
    GenBash(GenBash),
}
