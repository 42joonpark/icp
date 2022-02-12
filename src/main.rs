mod cli;
mod program;

use cli::Config;
use cli_42::SessionError;
use program::Program;

async fn run(prog: &mut Program) -> Result<(), SessionError> {
    let command = prog.config.command.to_owned();
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        // "ID" => prog.id().await?,
        "ME" => prog.me().await?,
        "EMAIL" => prog.email().await?,
        // "LOGIN" => prog.login().await?,
        // "POINT" => prog.correction_point().await?,
        // "CAMPUS" => prog.campus().await?,
        // "WALLET" => prog.wallet().await?,
        "COMMAND" => prog.config.list_available_commands(),
        _ => println!("Command `{}` not found", command),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), SessionError> {
    env_logger::init();

    let config = Config::new()?;

    let mut program = Program::new(config).await?;
    run(&mut program).await?;
    Ok(())
}
