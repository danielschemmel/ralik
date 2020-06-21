use anyhow::anyhow;

use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::sync::atomic::AtomicIsize;
use std::sync::{Arc, RwLock};

use crate::error::RuntimeError;
use crate::types::{GenericTypeBuilder, Type};
use crate::Value;

mod debug;
mod functions;
mod macros;

mod types;
pub use types::TypeHandle;

mod variables;

/**
The `Context` stores all types, free functions and global variables.

While it can be customized to a large extent, it is suggested to start with the default context, which contains working
types for the core types:
```rust
# use ralik::Context;
let context = Context::new();
```

The default context can then be extended for the specific use case:
```rust
# use ralik::{Context, Value};
# let context = Context::new();
// Define a global variable `$` of the Unit type `()`
context.insert_variable("$", Value::new_unit(&context).unwrap());
```

When starting with an empty context, the core types are not available by default:
```rust
# use ralik::{Context, Value};
let context = Context::new_empty();
Value::new_bool(&context, true).unwrap_err();
```
*/
#[derive(Clone)]
pub struct Context(Arc<ContextImpl>);

struct ContextImpl {
	arrays: RwLock<Option<GenericTypeCreator>>,
	tuples: RwLock<Option<GenericTypeCreator>>,

	types: RwLock<Vec<(Type, AtomicIsize)>>,
	names: RwLock<HashMap<String, Thing>>,
	macros: RwLock<HashMap<String, Macro>>,
}

pub type Function = fn(&Context, &[Value]) -> Result<Value, RuntimeError>;
pub type GenericTypeCreator = fn(&Context, &[&str]) -> Result<GenericTypeBuilder, anyhow::Error>;
pub type Macro = fn(&Context, &[Value]) -> Result<Value, RuntimeError>;

#[derive(Copy, Clone)]
pub(crate) struct TypeId(usize);

enum Thing {
	Variable(Value),
	Function(Function),
	Type(TypeId),
	Generic(Arc<Generic>),
}

struct Generic {
	name: String,
	creator: GenericTypeCreator,
}

impl Context {
	pub fn new() -> Self {
		let context = Self::new_empty();

		context.register_types(vec![crate::types::new_bool_type()]);
		context.register_types(vec![crate::types::new_char_type()]);
		context.register_types(vec![crate::types::new_integer_type()]);
		context.register_types(vec![crate::types::new_string_type()]);
		context.register_tuple_generic(crate::types::tuple_generic);
		context.register_array_generic(crate::types::array_generic);

		context.insert_macro("concat", |context, mut arguments| {
			let mut result = String::new();

			while !arguments.is_empty() {
				result.push_str(
					arguments[0]
						.as_string()
						.ok_or_else(|| anyhow!("`include_str!` takes exactly one argument of string type"))?,
				);
				arguments = &arguments[1..];
			}

			Ok(Value::new_string(context, result)?)
		});

		context.insert_macro("env", |context, arguments| {
			if arguments.len() != 1 {
				return Err(anyhow!("`env!` takes exactly one argument of string type").into());
			}
			let value = arguments[0]
				.as_string()
				.ok_or_else(|| anyhow!("`env!` takes exactly one argument of string type"))?;

			if let Ok(content) = std::env::var(value) {
				Ok(Value::new_string(context, content)?)
			} else {
				Err(anyhow!("Environment variable {} is not set", value).into())
			}
		});

		context.insert_macro("include", |context, arguments| {
			if arguments.len() != 1 {
				return Err(anyhow!("`include!` takes exactly one argument of string type").into());
			}
			let value = arguments[0]
				.as_string()
				.ok_or_else(|| anyhow!("`include!` takes exactly one argument of string type"))?;

			let content = read_to_string(value).map_err(|err| anyhow!(err))?;

			// FIXME: figure out why `anyhow!(err)` does not work
			Ok(crate::eval_str(&content, context).map_err(|err| anyhow!(err.to_string()))?)
		});

		context.insert_macro("include_bytes", |context, arguments| {
			if arguments.len() != 1 {
				return Err(anyhow!("`include_bytes!` takes exactly one argument of string type").into());
			}
			let value = arguments[0]
				.as_string()
				.ok_or_else(|| anyhow!("`include_bytes!` takes exactly one argument of string type"))?;

			let file = File::open(value).map_err(|err| anyhow!(err))?;
			let bytes = file
				.bytes()
				.map(|result| {
					result
						.map_err(|err| RuntimeError::from(anyhow!(err)))
						.and_then(|byte| Ok(Value::new_integer(context, byte)?))
				})
				.collect::<Result<Vec<Value>, RuntimeError>>()?;

			Ok(Value::new_array(
				context,
				&context
					.get_integer_type()
					.map_err(|err| crate::error::ValueCreationError::IntegerCreationError(err.into()))?,
				bytes,
			)?)
		});

		context.insert_macro("include_str", |context, arguments| {
			if arguments.len() != 1 {
				return Err(anyhow!("`include_str!` takes exactly one argument of string type").into());
			}
			let value = arguments[0]
				.as_string()
				.ok_or_else(|| anyhow!("`include_str!` takes exactly one argument of string type"))?;

			let content = read_to_string(value).map_err(|err| anyhow!(err))?;

			Ok(Value::new_string(context, content)?)
		});

		/*context.insert_macro("option_env", |context, arguments| {
			if arguments.len() != 1 {
				return Err(anyhow!("`option_env!` takes exactly one argument of string type").into());
			}
			let value = arguments[0]
				.as_string()
				.ok_or_else(|| anyhow!("`option_env!` takes exactly one argument of string type"))?;

			let option_type = context
				.get_option_type("std::string::String")
				.map_err(|err| anyhow!("Could not create option type: {}", err))?;
			if let Ok(content) = std::env::var(value) {
				Ok(Value::new_enum_tuple_variant(
					context,
					option_type.name(),
					"Some",
					Box::new([Value::new_string(context, content)?]) as Box<[Value]>,
				)?)
			} else {
				Ok(Value::new_enum_unit_variant(context, option_type.name(), "None")?)
			}
		});*/

		context.insert_macro("panic", |_context, arguments| {
			use std::fmt::Write;
			let mut message = "Call to `panic!(".to_owned();
			if !arguments.is_empty() {
				write!(message, "\n").unwrap();
			}
			for argument in arguments {
				write!(message, "  {:?},\n", argument).unwrap();
			}
			write!(message, ")").unwrap();
			Err(anyhow::anyhow!(message).into())
		});

		context.insert_macro("vec", |context, arguments| {
			if arguments.is_empty() {
				Err(anyhow::anyhow!("Empty `vec!` calls are currently not supported").into())
			} else {
				let type_0 = arguments[0].get_type();
				Ok(Value::new_array(
					context,
					type_0,
					arguments.iter().cloned().collect::<Box<[Value]>>(),
				)?)
			}
		});

		context
	}

	pub fn new_empty() -> Self {
		Context(Arc::new(ContextImpl {
			tuples: Default::default(),
			arrays: Default::default(),
			types: Default::default(),
			names: Default::default(),
			macros: Default::default(),
		}))
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
