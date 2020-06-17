use anyhow::anyhow;

use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::sync::{Arc, RwLock};

use crate::error::RuntimeError;
use crate::Value;

mod debug;
mod functions;
mod macros;
mod types;
mod variables;

pub type Function = fn(&Context, &[Value]) -> Result<Value, RuntimeError>;
pub type Macro = fn(&Context, &[Value]) -> Result<Value, RuntimeError>;

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
Value::new_unit(&context).unwrap_err();
```
*/
#[derive(Clone)]
pub struct Context(Arc<ContextImpl>);

struct ContextImpl {
	types: types::TypeContainer,
	variables: RwLock<HashMap<String, Value>>,
	functions: RwLock<HashMap<String, Function>>,
	macros: RwLock<HashMap<String, Macro>>,
}

impl Context {
	pub fn new() -> Self {
		let context = Self::new_empty();

		context.insert_type(crate::types::BoolType::new());
		context.insert_type(crate::types::CharType::new());
		context.insert_type(crate::types::IntegerType::new());
		context.insert_type(crate::types::StringType::new());

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
			types: Default::default(),
			variables: RwLock::new(HashMap::new()),
			functions: RwLock::new(HashMap::new()),
			macros: RwLock::new(HashMap::new()),
		}))
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
