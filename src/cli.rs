use ftapi::SessionError;
use clap::{crate_description, crate_name, crate_version, App, Arg};

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub list_commands: bool,
    pub page: Option<u32>,
}

impl Config {
    pub fn new() -> Result<Self, SessionError> {
        let matches = App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .arg(
                Arg::new("command")
                    .default_value("me")
                    .index(1)
                    .help("Command to execute"),
            )
            .arg(
                Arg::new("list_commands")
                    .short('c')
                    .help("List all available commands"),
            )
            .arg(
                Arg::new("page")
                    .short('p')
                    .long("page")
                    .takes_value(true)
                    .help("Page number"),
            )
            .get_matches();

        let command = matches.value_of("command").unwrap_or("me");
        let list_commands = matches.is_present("list_commands");
        let page = matches
            .value_of("page")
            .map(|p| p.parse::<u32>().unwrap_or_default());
        Ok(Config {
            command: String::from(command),
            list_commands,
            page,
        })
    }
}

pub fn list_available_commands() -> Result<(), SessionError> {
    println!("Available commands:");
    println!("  id");
    println!("  me");
    println!("  email");
    println!("  login");
    println!("  point");
    println!("  campus");
    println!("  wallet");
    Ok(())
}
