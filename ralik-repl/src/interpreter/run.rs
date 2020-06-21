use anyhow::Result;

use crate::cli::ReturnCode;
use crate::util::print_error_chain;

use super::create_context;

#[derive(structopt::StructOpt, Debug)]
pub struct Args {
	#[structopt(
		long = "dump-context",
		help = "Dumps the interpreter context at the beginning of the session."
	)]
	dump_context: bool,

	#[structopt(long = "no-$", help = "Skip registering the interpreter object `$`.")]
	no_interpreter_value: bool,

	#[structopt(help = "The command to run.")]
	command: String,
}

pub fn main(args: Args) -> Result<ReturnCode> {
	let context = create_context(!args.no_interpreter_value)?;

	if args.dump_context {
		println!("{:+#?}", context);
	}

	match ralik::eval_str(&args.command, &context) {
		Ok(expr) => {
			println!("{}", expr);
			Ok(ReturnCode::Success)
		}
		Err(err) => {
			print_error_chain(&err);
			Ok(ReturnCode::ScriptError)
		}
	}
}
