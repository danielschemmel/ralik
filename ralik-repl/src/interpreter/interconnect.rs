use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "$Interpreter")]
pub struct Interpreter;

impl Interpreter {
	pub fn new() -> Self {
		Self
	}

	pub fn register_type(context: &ralik::Context) {
		let mut interpreter_type = ralik::types::UnitStructType::new("$Interpreter");

		interpreter_type.insert_function("dump", |context, this_type, args| {
			let _this = args
				.get(0)
				.filter(|value| value.has_type(this_type))
				.ok_or_else(|| anyhow!("Interpreter member functions should only be called on interpreter object"))?;

			if args.len() != 2 {
				return Err(anyhow!("`$Interpreter::exit` takes 2 arguments ({} provided)", args.len()).into());
			}

			println!("{:+#?}", args[1]);

			Ok(ralik::Value::new_unit(context)?)
		});

		interpreter_type.insert_function("exit", |_context, this_type, args| {
			let _this = args
				.get(0)
				.filter(|value| value.has_type(this_type))
				.ok_or_else(|| anyhow!("Interpreter member functions should only be called on interpreter object"))?;

			match args.len() {
				0 | 1 => {
					std::process::exit(0);
				}
				2 => args[1]
					.as_i32()
					.map(std::process::exit)
					.ok_or_else(|| anyhow!("Argument to `$Interpreter::exit` must be a valid `i32` if it exists."))?,
				n => Err(anyhow!("`$Interpreter::exit` takes 1 or 2 arguments ({} provided)", n).into()),
			}
		});

		context.insert_type(interpreter_type);
	}
}
