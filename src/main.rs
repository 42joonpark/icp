use cli::Config;
use log::{debug, info};

pub mod authorize;
pub mod cli;
pub mod command;
pub mod structs;
use cli_42::CliError;
use structs::program::Program;

async fn run(prog: &mut Program, command: String) -> Result<(), CliError> {
    info!("run() Begin");
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        "WALLET" => {
            println!("Wallet: {}", prog.wallet().await?);
        }
        "EMAIL" => match prog.email().await {
            Ok(content) => {
                println!("Email: {}", content);
            }
            Err(error) => {
                println!("Error... {}", error);
            }
        },
        "ID" => {
            println!("ID: {}", prog.id().await?);
        }
        "POINT" => {
            println!("Correction Point: {}", prog.correction_point().await?);
        }
        "LOGIN" => {
            println!("Login: {}", prog.login().await?);
        }
        "CAMPUS" => {
            prog.campus().await?;
        }
        _ => {
            println!("Command not found");
        }
    }
    info!("run() End");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    info!("Staring Program");
    env_logger::init();

    let config = Config::new()?;
    debug!("{}", config.command);

    let mut program = Program::new().await?;
    run(&mut program, config.command.to_owned()).await?;
    info!("Quit Program");
    Ok(())
}
