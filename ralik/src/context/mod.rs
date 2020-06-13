use std::collections::hash_map::HashMap;
use std::sync::{Arc, RwLock};

use crate::{CallError, Type, Value};

mod debug;
mod functions;
mod macros;
mod types;
mod variables;

pub type Function = fn(&[Value]) -> Result<Value, CallError>;
pub type Macro = fn(&[Value]) -> Result<Value, CallError>;

#[derive(Clone)]
pub struct Context(Arc<ContextImpl>);

struct ContextImpl {
	types: RwLock<HashMap<String, Arc<dyn Type>>>,
	variables: RwLock<HashMap<String, Value>>,
	functions: RwLock<HashMap<String, Function>>,
	macros: RwLock<HashMap<String, Macro>>,
}

impl Context {
	pub fn new() -> Self {
		let context = Self::new_empty();

		context.insert_type(Arc::new(crate::types::BoolType::new()));
		context.insert_type(Arc::new(crate::types::CharType::new()));
		context.insert_type(Arc::new(crate::types::IntegerType::new()));
		context.insert_type(Arc::new(crate::types::StringType::new()));

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
