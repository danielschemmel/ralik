use std::collections::hash_map::HashMap;
use std::sync::{Arc, RwLock};

use crate::error::RuntimeError;
use crate::{TypeHandle, Value};

mod debug;
mod functions;
mod macros;
mod types;
mod variables;

pub type Function = fn(&[Value]) -> Result<Value, RuntimeError>;
pub type Macro = fn(&[Value]) -> Result<Value, RuntimeError>;

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
	types: RwLock<HashMap<String, TypeHandle>>,
	variables: RwLock<HashMap<String, Value>>,
	functions: RwLock<HashMap<String, Function>>,
	macros: RwLock<HashMap<String, Macro>>,
}

impl Context {
	pub fn new() -> Self {
		let context = Self::new_empty();

		context.insert_type(TypeHandle::new(crate::types::UnitType::new()));
		context.insert_type(TypeHandle::new(crate::types::BoolType::new()));
		context.insert_type(TypeHandle::new(crate::types::CharType::new()));
		context.insert_type(TypeHandle::new(crate::types::IntegerType::new()));
		context.insert_type(TypeHandle::new(crate::types::StringType::new()));

		context.insert_macro("panic", |arguments| {
			use std::fmt::Write;
			let mut message = "Call to `panic!(".to_string();
			if !arguments.is_empty() {
				write!(message, "\n").unwrap();
			}
			for argument in arguments {
				write!(message, "  {:?},\n", argument).unwrap();
			}
			write!(message, ")").unwrap();
			Err(anyhow::anyhow!(message).into())
		});

		context
	}

	pub fn new_empty() -> Self {
		Context(Arc::new(ContextImpl {
			types: RwLock::new(HashMap::new()),
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
