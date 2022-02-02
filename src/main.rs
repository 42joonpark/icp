use clap::{crate_name, crate_version, App, Arg};
use log::debug;

pub mod authorize;
pub mod command;
pub mod structs;
use cli_42::CliError;
use structs::program::Program;

async fn run(prog: &mut Program, command: &str) -> Result<(), CliError> {
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
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    env_logger::init();

    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about("42 cli")
        .arg(
            Arg::new("command")
                .required(true)
                .index(1)
                .help("Command to run"),
        )
        .get_matches();
    debug!("{}", cli.value_of("command").unwrap());

    let mut program = Program::new();
    program.init_program().await?;
    run(&mut program, cli.value_of("command").unwrap()).await?;
    Ok(())
}
