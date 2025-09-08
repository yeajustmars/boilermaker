use clap::Parser;

#[derive(Parser)]
pub(crate) struct Test {
    #[arg(short, long)]
    pub list: bool,
}
