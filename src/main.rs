use log::{debug, info};
use std::io::{self, Write};

pub mod authorize;
pub mod command;
pub mod structs;
use cli_42::CliError;
use structs::program::Program;

async fn run(prog: &mut Program) -> Result<(), CliError> {
    let reader = io::stdin();
    loop {
        let mut line = String::new();
        print!("42_cli > ");
        io::stdout().flush()?;
        let bytes = reader.read_line(&mut line);
        match bytes {
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
            Ok(bytes) => {
                if bytes == 0 {
                    println!("bye!");
                    break;
                }
            }
        }
        let command = line.trim().to_uppercase();
        info!("COMMAND: {}", command);
        match command.as_str() {
            "CLEAR" => command::clear(),
            "HELP" => command::help(),
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
            "WALLET" => {
                println!("Wallet: {}", prog.wallet().await?);
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
            "QUIT" => {
                println!("bye!!!👋👋👋👋");
                break;
            }
            _ => println!("42cli: command not found: {}", line.trim()),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    env_logger::init();
    dotenv::dotenv()?;

    let mut program = Program::new();
    program.init_program().await?;
    debug!("**** Program Status ****\n{:#?}", program);

    run(&mut program).await?;
    debug!("**** Program Status ****\n{:#?}", program);
    Ok(())
}
