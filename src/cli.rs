use clap::{crate_description, crate_name, crate_version, App, Arg};
use cli_42::SessionError;

#[derive(Debug, clap::Parser)]
pub struct Config {
    pub command: String,
    pub page: Option<u32>,
    commands: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, SessionError> {
        let arg_command = Arg::new("command")
            .default_value("command")
            .index(1)
            .possible_values(
                [
                    "command",
                    "id",
                    "me",
                    "email",
                    "login",
                    "point",
                    "wallet",
                    "blackhole",
                ]
                .iter(),
            )
            .takes_value(true)
            .help("Command to execute");
        let mut commands: Vec<String> = Vec::new();
        if let Some(val) = arg_command.get_possible_values() {
            for v in val {
                commands.push(v.get_name().to_string());
            }
        }
        let matches = App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .arg(arg_command)
            .arg(
                Arg::new("page")
                    .short('p')
                    .long("page")
                    .takes_value(true)
                    .help("Page number"),
            )
            .get_matches();

        let command = matches.value_of("command").unwrap_or("me");
        let page = matches
            .value_of("page")
            .map(|p| p.parse::<u32>().unwrap_or_default());
        Ok(Config {
            command: String::from(command),
            page,
            commands,
        })
    }

    pub fn list_available_commands(&self) {
        println!("Available commands:");
        for command in &self.commands {
            println!("\t{}", command);
        }
    }
}
