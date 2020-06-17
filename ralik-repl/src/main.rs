#![forbid(unsafe_code)]

mod cli;
mod interpreter;
mod stats;
mod util;

use anyhow::{Context, Result};
use cli::{Args, ReturnCode};

build_info::build_info!(fn build_info);

fn parse_arguments() -> Result<ReturnCode> {
	use structopt::clap::ErrorKind;
	use structopt::StructOpt;

	match Args::from_iter_safe(std::env::args_os()) {
		Ok(args) => cli::main(args),
		Err(e) => match e.kind {
			ErrorKind::VersionDisplayed => {
				println!("{}", e.message);
				Ok(ReturnCode::Success)
			}
			ErrorKind::HelpDisplayed => {
				println!("{}", e.message);
				Ok(ReturnCode::Success)
			}
			_ => {
				println!("{}", e.message);
				Ok(ReturnCode::ArgumentParsing)
			}
		},
	}
}

fn main() -> Result<()> {
	parse_arguments()
		.map(|return_code| std::process::exit(return_code as i32))
		.context("Uncaught error in main")
}
