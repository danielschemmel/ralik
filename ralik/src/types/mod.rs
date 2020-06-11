use std::collections::HashMap;
use std::fmt;

use super::eval::CallError;
use super::Value;

mod bool;
mod char;
mod integer;
mod string;

pub type MemberFunction = fn(&[Value]) -> Result<Value, CallError>;

pub struct Type {
	name: String,
	functions: HashMap<String, MemberFunction>,
}

impl Type {
	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn call(&self, name: &str, arguments: &[Value]) -> Result<Value, CallError> {
		self
			.functions
			.get(name)
			.ok_or_else(|| CallError::FunctionDoesNotExist {
				member_name: name.to_string(),
				type_name: self.name().to_string(),
			})
			.and_then(|function| function(arguments))
	}
}

impl fmt::Debug for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Type")
			.field("name", &self.name)
			.field(
				"functions",
				&FunctionNameListFormatter {
					functions: &self.functions,
				},
			)
			.finish()
	}
}

struct FunctionNameListFormatter<'a> {
	functions: &'a HashMap<String, MemberFunction>,
}
impl<'a> fmt::Debug for FunctionNameListFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list().entries(self.functions.keys()).finish()
	}
}
