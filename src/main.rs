mod cli;
mod program;

use cli::Config;
use cli_42::SessionError;
use program::Program;
use program::Command;

async fn run(prog: &mut Program) -> Result<(), SessionError> {
    let command = prog.config.command.to_owned();
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        "ME" => prog.run_program(Command::Me).await?,
        "ID" => prog.run_program(Command::Id).await?,
        "EMAIL" => prog.run_program(Command::Email).await?,
        "LOGIN" => prog.run_program(Command::Login).await?,
        "POINT" => prog.run_program(Command::CorrectionPoint).await?,
        "WALLET" => prog.run_program(Command::Wallet).await?,
        "BLACKHOLE" => prog.run_program(Command::Blackhole).await?,
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
