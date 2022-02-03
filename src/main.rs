pub mod authorize;
pub mod cli;
pub mod command;
pub mod structs;
use cli::Config;
use cli_42::CliError;
use structs::program::Program;

async fn run(prog: &mut Program, config: Config) -> Result<(), CliError> {
    let command = config.command.to_owned();
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
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    env_logger::init();

    let config = Config::new()?;
    let mut program = Program::new().await?;

    run(&mut program, config).await?;
    Ok(())
}
