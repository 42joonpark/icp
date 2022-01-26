use anyhow::Result;
use log::debug;
use std::io::{self, Write};

pub mod authorize;
pub mod command;
pub mod json;
pub mod structs;
use command::me;
use structs::program::Program;

async fn run(prog: &mut Program) -> Result<()> {
    let reader = io::stdin();
    // print welcome message
    // command::welcome_msg(prog).await?;
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
        // match command.as_str() {
        //     "CLEAR" => command::clear(),
        //     "ME" => me::load_info(prog).await?,
        //     "EMAIL" => command::email(prog),
        //     "ID" => command::id(prog),
        //     "WALLET" => command::wallet(prog),
        //     "LOGIN" => command::login(prog),
        //     "POINT" => command::correction_point(prog),
        //     "RELOAD" => command::reload_me(prog).await?,
        //     "HELP" | "COMMAND" => command::help(),
        //     "QUIT" => {
        //         println!("bye!!!👋👋👋👋");
        //         break;
        //     }
        //     _ => println!("42cli: command not found: {}", line.trim()),
        // }
        match command.as_str() {
            "CLEAR" => command::clear(),
            // "ME" => me::load_info(prog).await?,
            "EMAIL" => {
                println!("Email: {}", prog.email().await?);
            },
            "ID" => {
                println!("ID: {}", prog.id().await?);
            },
            "WALLET" => {
                println!("Wallet: {}", prog.wallet().await?);
            },
            "POINT" => {
                println!("Correction Point: {}", prog.correction_point().await?);
            },
            "LOGIN" => {
                println!("Login: {}", prog.login().await?);
            },
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
    println!("{:#?}", program);

    run(&mut program).await?;
    Ok(())
}
