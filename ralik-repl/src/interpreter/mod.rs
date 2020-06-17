use anyhow::{anyhow, Result};

use crate::util::print_error_chain;

mod interconnect;
use interconnect::Interpreter;

pub mod repl;
pub mod run;

fn create_context(register_interconnect: bool) -> Result<ralik::Context> {
	let context = ralik::Context::new();

	if register_interconnect {
		Interpreter::register_type(&context);

		context.insert_variable(
			"$",
			ralik::Value::from_serde(&context, &Interpreter::new(), "$Interpreter")
				.map_err(|err| print_error_chain(&err))
				.unwrap(),
		);
	}

	context.insert_function("exit", |_context, args| match args.len() {
		0 => std::process::exit(0),
		1 => args[0]
			.as_i32()
			.map(std::process::exit)
			.ok_or_else(|| anyhow!("Argument to `exit` must be a valid `i32` if it exists."))?,
		n => Err(anyhow!("`exit` takes 0 or 1 arguments ({} provided)", n).into()),
	});

	Ok(context)
}
