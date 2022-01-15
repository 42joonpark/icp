use anyhow::{Context, Result};
use log::{info, warn, debug};
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
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
	env_logger::init();
	let args = Cli::from_args();
	let path = args.path.as_os_str().to_str().unwrap();
	let f = File::open(&args.path)
			.with_context(|| format!("could not read file {}", path))?;

	let stdout = io::stdout();
	let mut writer = stdout.lock();
	let mut reader = BufReader::new(f);
	loop {
		let mut line = String::new();
		let len = reader.read_line(&mut line)?;
		if len == 0 {
			break;
		}
		if line.contains(&args.pattern) {
			writeln!(writer, "{}", line)
			.with_context(|| format!("could not write to writebuf"))?;
		}
	}
	debug!("Main End");
	Ok(())
}