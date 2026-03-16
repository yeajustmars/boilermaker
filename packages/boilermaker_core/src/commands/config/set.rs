use clap::Parser;
use color_eyre::Result;

//use crate::{config, state::AppState};
use crate::state::AppState;

#[derive(Debug, Parser)]
pub struct Set {
    #[arg(required = true, num_args = 1.., help = "k=v pairs to set in system config")]
    pub kv: Vec<String>,
}

#[tracing::instrument]
pub async fn set(app_state: &AppState, cmd: &Set) -> Result<()> {
    // 1. get config on disk
    // 2. fail if not config, nothing to set
    // 3. update config on disk
    // 4. update config in memory

    //let mut config = config::get_system_config(config_path)
    println!("PICK UP HERE");

    Ok(())
}
