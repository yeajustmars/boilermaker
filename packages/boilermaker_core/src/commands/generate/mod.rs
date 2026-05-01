use clap::Subcommand;

pub mod blank;
pub use blank::{Blank, blank};

const BLANK_HELP: &str = "
Examples:
    boil generate blank my-template Rust Python JavaScript

    boil generate blank my-template Rust --dir=/home/my-user
";

#[derive(Subcommand)]
pub enum Generate {
    #[command(
        about = "Generate a blank, bare-bones template",
        after_help = BLANK_HELP
    )]
    Blank(Blank),
}
