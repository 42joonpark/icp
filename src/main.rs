use anyhow::Result;
use log::debug;
use std::io::{self, Write};
use std::{error};

pub mod authorize;
pub mod structs;
pub mod json;
pub mod command;
use structs::program::Program;
use command::me;

async fn run(prog: &mut Program) -> Result<(), Box<dyn error::Error>> {
    let reader = io::stdin();
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
            Ok(size) => {
                if size == 0 {
                    println!("bye!");
                    break;
                }
            }
        }
        let command = line.trim().to_uppercase();
        debug!("COMMAND: {}", command);
        match command.as_str() {
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
            _ => {}
        }
        println!("{}", line);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let mut program = Program::new();
    program.init_program().await?;

    run(&mut program).await?;
    Ok(())
}
