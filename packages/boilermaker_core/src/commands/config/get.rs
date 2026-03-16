use clap::Parser;
use color_eyre::Result;

use crate::{state::AppState, util::output::print_table};

#[derive(Debug, Parser)]
pub struct Get {
    pub key: Option<String>,
}

#[tracing::instrument]
pub async fn get(app_state: &AppState, cmd: &Get) -> Result<()> {
    let m = app_state.sys_config.to_hashmap();

    if let Some(key) = &cmd.key {
        match m.get(key) {
            Some(value) => println!("{:#?}", value),
            None => println!("Key `{}` not found in system config.", key),
        }
    } else {
        print_table(m.into_iter());
    }

    Ok(())
}
