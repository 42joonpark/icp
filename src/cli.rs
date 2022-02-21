use crate::error::CliError;
use clap::{crate_description, crate_name, crate_version, App, Arg};

// TODO:
// - Add a --detail flag to print more information about the result
// TODO:
// - add method functions
#[derive(Clone, Debug, clap::Parser)]
pub struct Cli {
    pub command: String,
    pub page: Option<u32>,
    pub user: Option<String>,
    pub detail: Option<bool>,
    commands: Vec<String>,
}

impl Cli {
    pub fn new() -> Result<Self, CliError> {
        let arg_command = Arg::new("command")
            .default_value("command")
            .index(1)
            .possible_values(
                [
                    "command",
                    "id",
                    "me",
                    "email",
                    "event",
                    "login",
                    "point",
                    "level",
                    "location",
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
                Arg::new("user")
                    .short('u')
                    .long("user")
                    .takes_value(true)
                    .help("User login"),
            )
            .arg(
                Arg::new("detail")
                    .short('d')
                    .long("detail")
                    .takes_value(false)
                    .help("Print more information about the result"),
            )
            .get_matches();

        let command = matches.value_of("command").unwrap_or("me");
        let page = None;
        let user = matches.value_of("user").map(|u| u.to_string());
        let detail = matches.is_present("detail");
        Ok(Cli {
            command: String::from(command),
            page,
            user,
            detail: Some(detail),
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
