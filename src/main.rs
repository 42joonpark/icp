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
        "ID" => prog.id().await?,
        "ME" => prog.me().await?,
        "EMAIL" => prog.email().await?,
        "LOGIN" => prog.login().await?,
        "POINT" => prog.correction_point().await?,
        "CAMPUS" => prog.campus().await?,
        "WALLET" => prog.wallet().await?,
        _ => println!("Command `{}` not found", command),
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
