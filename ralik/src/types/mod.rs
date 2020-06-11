use std::collections::HashMap;

use super::eval::CallError;
use super::Value;

mod bool;
mod char;
mod integer;
mod string;

pub struct Type {
	name: String,
	functions: HashMap<String, fn(Vec<Value>) -> Result<Value, CallError>>,
	#[allow(dead_code)]
	fields: HashMap<String, Value>,
}

impl Type {
	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn call(&self, name: &str, arguments: Vec<Value>) -> Result<Value, CallError> {
		self
			.functions
			.get(name)
			.ok_or_else(|| CallError::FunctionDoesNotExist {
				name: name.to_string(),
				type_name: self.name().to_string(),
			})
			.and_then(|function| function(arguments))
	}
}
