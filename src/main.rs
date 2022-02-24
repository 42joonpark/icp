mod error;
mod client;

use client::Client;
use error::CliError;
#[allow(unused_imports)]
use log::{self, debug, info};

async fn wrapped_main() -> Result<(), CliError> {
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