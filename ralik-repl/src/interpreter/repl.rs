use anyhow::Result;
use rustyline::config::{Builder as EditorBuilder, CompletionType};
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
}

const PROMPT: &str = "> ";

pub fn main(args: Args) -> Result<ReturnCode> {
	let context = create_context(!args.no_interpreter_value)?;

	if args.dump_context {
		println!("{:+#?}", context);
	}

	let editor_config = EditorBuilder::new()
		.completion_type(CompletionType::List)
		.tab_stop(2)
		.build();
	let mut editor = Editor::<()>::with_config(editor_config);
	loop {
		match editor.readline(PROMPT) {
			Ok(line) => {
				editor.add_history_entry(line.as_str());
				match ralik::eval_str(&line, &context) {
					Ok(expr) => {
						println!("{}", expr);
					}
					Err(err) => {
						print_error_chain(&err);
					}
				}
			}
			Err(ReadlineError::Interrupted) => {
				// just reset the prompt
			}
			Err(ReadlineError::Eof) => {
				if atty::is(atty::Stream::Stdin) {
					println!("exit()");
				}
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}

	Ok(ReturnCode::Success)
}
