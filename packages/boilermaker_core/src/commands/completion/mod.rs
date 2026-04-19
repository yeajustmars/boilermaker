use clap::Subcommand;

pub mod generate;
pub use generate::{GenBash, gen_bash};

//pub mod set;
//pub use set::{Set, set};

#[derive(Subcommand)]
pub enum Completion {
    #[command(about = "Get BASH autocomplete for Boilermaker")]
    GenBash(GenBash),
}
