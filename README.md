# <span style="color: rgb(255, 111, 122)"> icp </span>
Intra Command line Program \
view your intra information on CLI

## How to use
1. Generate client_id and client_secret at intra.\
https://profile.intra.42.fr/oauth/applications/new  \
set redirect_url to "http://localhost:8080"
2. create `config.toml` file inside user config directory and put your client_id, client_secret and login. (Mac: $HOME/Library/Application Support) \
	client_id="your client_id" \
	client_secret="your client_secret" \
	login="Your intra login ex)joonpark"
3. `cli_42 --help` for help.
4. `cli_42 command` to see available commands.
5. `cli_42 [options] [command]` to run command.

### Log
to see log \
use `RUST_LOG=info` before `cargo run`\
`RUST_LOG=info cli_42 [options] [command]`

#### Available log
1. error - not used
2. warn
3. info
4. debug
5. trace - not used

## Commands
### blackhole
Show user's remaining blackhole days.
### email
Shows user email
### event
Shows campus events
### level
Show user level
### location
Show user location in the cluster
### login
Show user intra id
### id
Shows user id
### point
Shows user correction point
### wallet
Shows user wallet


## Options
### --detail(-d)
Show detail information.
### --user(-u)
Specify user login.\
ex) `./cli_42 --user=joonpark`
