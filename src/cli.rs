use crate::error::CliError;
use clap::{crate_description, crate_name, crate_version, App, Arg};

#[derive(Clone, Debug)]
pub struct Cli {
    pub _command: String,
    pub _page: Option<u32>,
    pub _user: Option<String>,
    pub _detail: bool,
    pub _me: bool,
    pub _id: bool,
    pub _grade: bool,
    pub _level: bool,
    pub _login: bool,
    pub _point: bool,
    pub _wallet: bool,
    pub _location: bool,
    pub _blackhole: bool,
    pub _run: bool,
}

impl Cli {
    pub fn new() -> Result<Self, CliError> {
        let arg_command = Arg::new("command")
            .default_value("command")
            .index(1)
            .possible_values(["command", "me", "event", "email"].iter())
            .takes_value(true)
            .help("Command to execute");
        let matches = App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .arg(&arg_command)
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
                Arg::new("grade")
                    .short('g')
                    .long("grade")
                    .takes_value(false)
                    .help("Print user grade"),
            )
            .arg(
                Arg::new("level")
                    .short('v')
                    .long("level")
                    .takes_value(false)
                    .help("Print user level"),
            )
            .arg(
                Arg::new("login")
                    .short('l')
                    .long("login")
                    .takes_value(false)
                    .help("Print user intra login(name)"),
            )
            .arg(
                Arg::new("point")
                    .short('p')
                    .long("point")
                    .takes_value(false)
                    .help("Print user point"),
            )
            .arg(
                Arg::new("wallet")
                    .short('w')
                    .long("wallet")
                    .takes_value(false)
                    .help("Print user wallet"),
            )
            .arg(
                Arg::new("location")
                    .short('o')
                    .long("location")
                    .takes_value(false)
                    .help("Print user location"),
            )
            .arg(
                Arg::new("blackhole")
                    .short('b')
                    .long("blackhole")
                    .takes_value(false)
                    .help("Print user blackhole"),
            )
            .get_matches();

        let _command = matches.value_of("command").unwrap_or("me");
        let _page = None;
        let _user = matches.value_of("user").map(|u| u.to_string());
        let _detail = matches.is_present("detail");
        let _id = matches.is_present("id");
        let _login = matches.is_present("login");
        let _point = matches.is_present("point");
        let _level = matches.is_present("level");
        let _location = matches.is_present("location");
        let _wallet = matches.is_present("wallet");
        let _grade = matches.is_present("grade");
        let _blackhole = matches.is_present("blackhole");
        let mut _run = true;

        if _command == "command" {
            println!("--- Available Commands ---");
            let mut _commands: Vec<String> = Vec::new();
            if let Some(val) = arg_command.get_possible_values() {
                for v in val {
                    println!("{}", v.get_name());
                }
            }
            _run = false;
        }
        Ok(Cli {
            _command: String::from(_command),
            _page,
            _user,
            _detail,
            // _commands,
            _me: !(_id || _login || _point || _level || _location || _wallet || _blackhole),
            _id,
            _login,
            _point,
            _level,
            _location,
            _wallet,
            _grade,
            _blackhole,
            _run,
        })
    }
}

impl Cli {
    pub fn run(&self) -> bool {
        self._run
    }
}
