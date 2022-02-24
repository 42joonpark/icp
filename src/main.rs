mod cli;
mod client;
mod error;

use client::Client;
use error::CliError;
#[allow(unused_imports)]
use log::{self, debug, info};

async fn wrapped_main() -> Result<(), CliError> {
    let config = cli::Cli::new()?;
    if config.run() {
        return Ok(());
    }
    let c = Client::new().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    match wrapped_main().await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
