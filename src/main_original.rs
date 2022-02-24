mod cli;
mod error;
mod program;
mod results;
mod session;
mod token;

use cli::Cli;
use error::CliError;
use program::Command;
use program::Program;

// TODO:
// Event 만들어주는 library 찾아보기 예) enum_derive? strum?
async fn run(prog: &mut Program) -> Result<(), CliError> {
    let command = prog.config.command.to_owned();
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        "ME" => prog.run_program(Command::Me).await?,
        "EMAIL" => prog.run_program(Command::Email).await?,
        "EVENT" => prog.run_program(Command::Event).await?,
        "COMMAND" => prog.config.list_available_commands(),
        _ => println!("Command `{}` not found", command),
    }
    Ok(())
}

async fn wrapped_main() -> Result<(), CliError> {
    let config = Cli::new()?;

    let mut program = Program::new(config.clone()).await?;
    run(&mut program).await?;
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
