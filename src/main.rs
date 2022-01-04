use core::panic;
use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use serde::de::value::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
	// The pattern to look for
	pattern: String,
	// The path to the file to read
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() {
	let args = Cli::from_args();
	let f = File::open(&args.path).unwrap();
	let mut reader = BufReader::new(f);

	loop {
		let mut line = String::new();
		let len = reader.read_line(&mut line).unwrap();
		if len == 0 {
			break;
		}
		if line.contains(&args.pattern) {
			println!("{}", line);
		}
	}
}