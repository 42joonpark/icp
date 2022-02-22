# <span style="color: rgb(255, 111, 122)"> icp </span>
Intra Command line Program \
view your intra information on CLI

## How to use
1. Generate client_id and client_secret at intra.\
https://profile.intra.42.fr/oauth/applications/new  \
set redirect_url to "http://localhost:8080"
2. create `config.toml` file inside user config directory and put your client_id, client_secret and login. (Mac: $HOME/Library/Application Support) \
	login="Your intra login ex)joonpark" \
	[session] \
	client_id="your client_id" \
	client_secret="your client_secret" \
3. `icp --help` for help.
4. `icp command` to see available commands.
5. `icp [options] [command]` to run command.

### Log
to see log \
use `RUST_LOG=info` before `cargo run`\
`RUST_LOG=info icp [options] [command]`

#### Available log
1. error - not used
2. warn
3. info
4. debug
5. trace - not used

## Commands
### me
Print user information
### email
Print user email
### event
Print campus events


## Options
### -b, --blackhole
Print user blackhole
### -d, --detail
Print more information about the result
### -g, --grade
Print user grade
### -h, --help
Print help information
### -i, --id
Print user intra id(number)
### -l, --login
Print user intra login(name)
### -o, --location
Print user location
### -p, --point
Print user point
### -u, --user <user>
User login
### -v, --level
Print user level
### -V, --version
Print version information
### -w, --wallet
Print user wallet