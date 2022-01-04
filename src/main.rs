use std::io::{BufReader, BufRead};
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
	// The pattern to look for
	pattern: String,
	// The path to the file to read
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::from_args();
	let f = File::open(&args.path)?;

	let mut reader = BufReader::new(f);
	loop {
		let mut line = String::new();
		let len = reader.read_line(&mut line)?;
		if len == 0 {
			break;
		}
		if line.contains(&args.pattern) {
			println!("{}", line);
		}
	}
	Ok(())
}