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

// Common
impl Type {
	pub fn name(&self) -> &str {
		&self.name
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

// Functions
impl Type {
	pub fn get_function(&self, key: &str) -> Option<&MemberFunction> {
		self.functions.get(key)
	}

	pub fn get_function_mut(&mut self, key: &str) -> Option<&mut MemberFunction> {
		self.functions.get_mut(key)
	}

	pub fn insert_function(&mut self, key: String, value: MemberFunction) -> Option<MemberFunction> {
		self.functions.insert(key, value)
	}

	pub fn remove_function(&mut self, key: &str) -> Option<(String, MemberFunction)> {
		self.functions.remove_entry(key)
	}
}
