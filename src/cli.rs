use crate::error::CliError;
use clap::{crate_description, crate_name, crate_version, App, Arg};

// TODO:
// - Add a --detail flag to print more information about the result
// TODO:
// - add method functions -> ??
#[derive(Clone, Debug)]
pub struct Cli {
    pub command: String,
    pub page: Option<u32>,
    pub user: Option<String>,
    commands: Vec<String>,
    pub detail: bool,
    pub id: bool,
    pub email: bool,
    pub login: bool,
    pub point: bool,
    pub level: bool,
    pub location: bool,
    pub wallet: bool,
    pub blackhole: bool,
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
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .takes_value(false)
                    .help("Print user intra id(number)"),
            )
            .arg(
                Arg::new("email")
                    .short('e')
                    .long("email")
                    .takes_value(false)
                    .help("Print user email"),
            )
            .arg(
                Arg::new("login")
                    .short('l')
                    .long("login")
                    .takes_value(false)
                    .help("Print user login"),
            )
            .arg(
                Arg::new("point")
                    .short('p')
                    .long("point")
                    .takes_value(false)
                    .help("Print user point"),
            )
            .arg(
                Arg::new("level")
                    .short('v')
                    .long("level")
                    .takes_value(false)
                    .help("Print user level"),
            )
            .arg(
                Arg::new("location")
                    .short('o')
                    .long("location")
                    .takes_value(false)
                    .help("Print user location"),
            )
            .arg(
                Arg::new("wallet")
                    .short('w')
                    .long("wallet")
                    .takes_value(false)
                    .help("Print user wallet"),
            )
            .arg(
                Arg::new("blackhole")
                    .short('b')
                    .long("blackhole")
                    .takes_value(false)
                    .help("Print user blackhole"),
            )
            .get_matches();

        let command = matches.value_of("command").unwrap_or("me");
        let page = None;
        let user = matches.value_of("user").map(|u| u.to_string());
        let detail = matches.is_present("detail");
        let id = matches.is_present("id");
        let email = matches.is_present("email");
        let login = matches.is_present("login");
        let point = matches.is_present("point");
        let level = matches.is_present("level");
        let location = matches.is_present("location");
        let wallet = matches.is_present("wallet");
        let blackhole = matches.is_present("blackhole");
        Ok(Cli {
            command: String::from(command),
            page,
            user,
            detail,
            commands,
            id,
            email,
            login,
            point,
            level,
            location,
            wallet,
            blackhole,
        })
    }

    pub fn list_available_commands(&self) {
        println!("Available commands:");
        for command in &self.commands {
            println!("\t{}", command);
        }
    }
}
