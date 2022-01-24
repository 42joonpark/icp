use anyhow::Result;
use log::debug;
use std::io::{self, Write};
use clap::{App, Arg};

pub mod authorize;
pub mod structs;
pub mod json;
pub mod command;
use structs::program::Program;
use command::me;

async fn run(prog: &mut Program) -> Result<()> {
    let reader = io::stdin();
    // print welcome message
    command::welcome_msg(prog).await?;
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
        debug!("COMMAND: {}", command);
        match command.as_str() {
            "CLEAR" => command::clear(),
            "ME" => me::load_info(prog).await?,
            "EMAIL" => command::email(prog),
            "ID" => command::id(prog),
            "WALLET" => command::wallet(prog),
            "LOGIN" => command::login(prog),
            "POINT" => command::correction_point(prog),
            "RELOAD" => command::reload_me(prog).await?,
            "HELP" | "COMMAND" => command::help(),
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
async fn main() -> Result<()> {
    env_logger::init();
         
    let mut program = Program::new();
    program.init_program().await?;

    run(&mut program).await?;
    Ok(())
}
