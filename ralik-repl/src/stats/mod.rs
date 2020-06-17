use anyhow::{anyhow, Result};

use crate::cli::ReturnCode;

mod stats;
use stats::Stats;

#[derive(structopt::StructOpt, Debug)]
pub struct Args {
	#[structopt(short = "f", long = "format", help="Format to display stats in", default_value = "human", possible_values = &["human", "json"])]
	format: Format,
}

#[derive(Copy, Clone, Debug)]
pub enum Format {
	Human,
	Json,
}

impl std::str::FromStr for Format {
	type Err = anyhow::Error;
	fn from_str(string: &str) -> Result<Self, Self::Err> {
		match string {
			"human" => Ok(Format::Human),
			"json" => Ok(Format::Json),
			_ => Err(anyhow!("The given format {:?} is not supported.", string)),
		}
	}
}

pub fn main(args: Args) -> Result<ReturnCode> {
	let stats = Stats::new();

	match args.format {
		Format::Human => println!("{}", stats),
		Format::Json => serde_json::to_writer_pretty(std::io::stdout().lock(), &stats)?,
	}

	Ok(ReturnCode::Success)
}
