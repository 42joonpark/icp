# <span style="color: rgb(255, 111, 122)"> cli_42_info </span>
CLI which prints 42 personal info

## How to use
1. Generate client_id and client_secret at intra.\
https://profile.intra.42.fr/oauth/applications/new 
2. create .env file inside "42_cli" directory
3. put your client_id and client_secret as below. \
	client_id="your client_id" \
	client_secret="your client_secret"
4. enter "help" or "command" to see list of commands

### Log
RUST_LOG=info

1. info
2. debug
3. warn

## Commands
### email
prints user email
### help
prints help
### id
prints user id
### login
prints user intra id
### point
prints user correction point
### reload
reload personal data
### wallet
prints user wallet
