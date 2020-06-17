use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Enumerat0r {
	Nuple(),
	Tuple(i32),
	Duple(String, Vec<i64>),
	Struct { foo: char },
	EmptyStruct {},
	Unit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Uniter;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NTupl();

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NStruc{}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Intz(i8, i16, i32, i64);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "$Interpreter")]
pub struct Interpreter {
	foo1: (),
	foo2: Uniter,
	foo3: NTupl,
	foo4: NStruc,
	bar1: (i8, i16, i32, i64),
	bar2: Intz,
	numerator1: Enumerat0r,
	numerator2: Enumerat0r,
	numerator3: Enumerat0r,
	numerator4: Enumerat0r,
	numerator5: Enumerat0r,
	numerator6: Enumerat0r,
}

impl Interpreter {
	pub fn new() -> Self {
		Self {
			foo1: (),
			foo2: Uniter,
			foo3: NTupl(),
			foo4: NStruc{},
			bar1: (8, 16, 32, 64),
			bar2: Intz(8, 16, 32, 64),
			numerator1: Enumerat0r::Nuple(),
			numerator2: Enumerat0r::Tuple(7),
			numerator3: Enumerat0r::Duple("foo".into(), vec![b'b' as i64, b'a' as i64, b'r' as i64]),
			numerator4: Enumerat0r::Struct { foo: '#' },
			numerator5: Enumerat0r::EmptyStruct {},
			numerator6: Enumerat0r::Unit,
		}
	}

	pub fn register_type(context: &ralik::Context) {
		let unit_type = context
			.get_unit_type()
			.map_err(|err| super::print_error_chain(&err))
			.unwrap();

		let integer_type = context
			.get_integer_type()
			.map_err(|err| super::print_error_chain(&err))
			.unwrap();

		let char_type = context
			.get_char_type()
			.map_err(|err| super::print_error_chain(&err))
			.unwrap();

		let string_type = context
			.get_string_type()
			.map_err(|err| super::print_error_chain(&err))
			.unwrap();

		let array_of_integer_type = context
			.get_array_type("Integer")
			.map_err(|err| super::print_error_chain(&err))
			.unwrap();

		let uniter = context.insert_type(ralik::types::UnitStructType::new("Uniter"));

		let ntupl = context.insert_type(ralik::types::TupleStructType::new("NTupl", []));
		
		let nstruc = context.insert_type(ralik::types::StructType::new("NStruc", (vec![] as Vec<(String, ralik::types::TypeHandle)>).into_iter()));

		let tuple_i_i_i_i_type = context
			.get_tuple_type(["Integer", "Integer", "Integer", "Integer"].iter())
			.map_err(|err| super::print_error_chain(&err))
			.unwrap();

		let intz = context.insert_type(ralik::types::TupleStructType::new(
			"Intz",
			vec![
				integer_type.clone(),
				integer_type.clone(),
				integer_type.clone(),
				integer_type.clone(),
			],
		));

		let enumerat0r_type = context.insert_type(ralik::types::EnumType::new(
			"Enumerat0r",
			vec![
				ralik::types::Variant::Tuple("Nuple".into(), Box::new([])),
				ralik::types::Variant::Tuple("Tuple".into(), Box::new([integer_type.clone()])),
				ralik::types::Variant::Tuple("Duple".into(), Box::new([string_type.clone(), array_of_integer_type])),
				ralik::types::Variant::Struct(
					"EmptyStruct".into(),
					vec![].into_iter().collect(),
					Box::new([char_type.clone()]),
				),
				ralik::types::Variant::Struct(
					"Struct".into(),
					vec![("foo".into(), 0)].into_iter().collect(),
					Box::new([char_type.clone()]),
				),
				ralik::types::Variant::Unit("Unit".into()),
			]
			.into_iter(),
		));
		let mut interpreter_type = ralik::types::StructType::new(
			"$Interpreter",
			vec![
				("foo1", unit_type.clone()),
				("foo2", uniter.clone()),
				("foo3", ntupl.clone()),
				("foo4", nstruc.clone()),
				("bar1", tuple_i_i_i_i_type.clone()),
				("bar2", intz.clone()),
				("numerator1", enumerat0r_type.clone()),
				("numerator2", enumerat0r_type.clone()),
				("numerator3", enumerat0r_type.clone()),
				("numerator4", enumerat0r_type.clone()),
				("numerator5", enumerat0r_type.clone()),
				("numerator6", enumerat0r_type.clone()),
			]
			.into_iter(),
		);

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
