use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Enumerat0r {
	Tuple(i32),
	/*Struct {
		foo: char,
	},*/
	Unit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "$Interpreter")]
pub struct Interpreter;

impl Interpreter {
	pub fn new() -> Self {
		Self // { test: Enumerat0r::Unit }
	}

	pub fn register_type(context: &ralik::Context) {
		/*let unit_type = context
		.get_unit_type()
		.map_err(|err| super::print_error_chain(&err))
		.unwrap();*/

		/*let t1 = context
		.get_tuple_type(&["Integer"])
		.map_err(|err| super::print_error_chain(&err))
		.unwrap();*/
		// let t2 = context.insert_type(ralik::types::StructType::new(name, fields));

		/*let enumerat0r_type = context.insert_type(ralik::types::EnumType::new(
			"Enumerat0r",
			vec![
				("Tuple", ralik::types::Variant::Tuple(Box::new([t1]))),
				("Unit", ralik::types::Variant::Unit),
			]
			.into_iter(),
		));
		let mut interpreter_type =
			ralik::types::StructType::new("$Interpreter", vec![("test", enumerat0r_type)].into_iter());*/
		let mut interpreter_type = ralik::types::StructType::new_unit("$Interpreter");

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
