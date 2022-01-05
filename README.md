# <span style="color: rgb(255, 111, 122)"> cli_42_info </span>
CLI which prints 42 personal info

## How to use
### General usage
1. Generate client_id and client_secret at intra.\
https://profile.intra.42.fr/oauth/applications/new 
2. make .env file at "cli_42_info" directory
3. enter your client_id and client_secret as below. \
	client_id="your client_id" \
	client_secret="your client_secret"
4. enter "help" or "commands" to see commands

### Log
RUST_LOG=info cargo run

1. info
2. debug
3. warn

## Commands (Not implemented yet)
### blackhole
prints remaining blackhole
### commands
prints available commands
### help
prints help
### set
change setting. ex) client_id
