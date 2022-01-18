# <span style="color: rgb(255, 111, 122)"> 42_cli </span>
view your intra information on CLI

## How to use
1. Generate client_id and client_secret at intra.\
https://profile.intra.42.fr/oauth/applications/new 
2. create .env file inside "42_cli" directory and put your client_id and client_secret.\
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
print user email
### help
print help
### id
print user id
### login
print user intra id
### point
print user correction point
### reload
reload personal data
### wallet
print user wallet
