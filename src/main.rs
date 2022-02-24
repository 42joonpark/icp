mod cli;
mod client;
mod error;
mod session;
mod program;
mod results;

use client::Client;
use error::CliError;
use program::Program;

#[allow(unused_imports)]
use log::{self, debug, info};

async fn wrapped_main() -> Result<(), CliError> {
    let cli = cli::Cli::new()?;
    if !cli.run() {
        return Ok(());
    }
    let client = Client::new().await?;
    let program = Program::new(client, cli);
    program.me().await?;
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
