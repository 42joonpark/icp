pub mod authorize;
pub mod cli;
pub mod error;
pub mod structs;
use cli::Config;
use error::CliError;
use structs::program::Program;

async fn run(prog: &mut Program) -> Result<(), CliError> {
    let command = prog.config.command.to_owned();
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        "ID" => prog.id().await?,
        "ME" => prog.me().await?,
        "EMAIL" => prog.email().await?,
        "LOGIN" => prog.login().await?,
        "POINT" => prog.correction_point().await?,
        "CAMPUS" => prog.campus().await?,
        "WALLET" => prog.wallet().await?,
        "COMMAND" => prog.config.list_available_commands(),
        _ => println!("Command `{}` not found", command),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    env_logger::init();

    let config = Config::new()?;

    let mut program = Program::new(config).await?;
    run(&mut program).await?;
    Ok(())
}
