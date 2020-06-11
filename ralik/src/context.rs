use std::collections::HashMap;

use super::eval::CallError;
use super::Value;

pub type Function = fn(&[Value]) -> Result<Value, CallError>;
pub type Macro = fn(&[Value]) -> Result<Value, CallError>;

pub struct Context {
	variables: HashMap<String, Value>,
	functions: HashMap<String, Function>,
	macros: HashMap<String, Macro>,
}

// Common
impl Context {
	pub fn new() -> Self {
		Context {
			variables: HashMap::new(),
			functions: HashMap::new(),
			macros: HashMap::new(),
		}
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

// Variables
impl Context {
	pub fn get_variable(&self, key: &str) -> Option<&Value> {
		self.variables.get(key)
	}

	pub fn get_variable_mut(&mut self, key: &str) -> Option<&mut Value> {
		self.variables.get_mut(key)
	}

	pub fn insert_variable(&mut self, key: String, value: Value) -> Option<Value> {
		self.variables.insert(key, value)
	}

	pub fn remove_variable(&mut self, key: &str) -> Option<(String, Value)> {
		self.variables.remove_entry(key)
	}
}

// Functions
impl Context {
	pub fn get_function(&self, key: &str) -> Option<&Function> {
		self.functions.get(key)
	}

	pub fn get_function_mut(&mut self, key: &str) -> Option<&mut Function> {
		self.functions.get_mut(key)
	}

	pub fn insert_function(&mut self, key: String, value: Function) -> Option<Function> {
		self.functions.insert(key, value)
	}

	pub fn remove_function(&mut self, key: &str) -> Option<(String, Function)> {
		self.functions.remove_entry(key)
	}
}

// Macros
impl Context {
	pub fn get_macro(&self, key: &str) -> Option<&Macro> {
		self.macros.get(key)
	}

	pub fn get_macro_mut(&mut self, key: &str) -> Option<&mut Macro> {
		self.macros.get_mut(key)
	}

	pub fn insert_macro(&mut self, key: String, value: Macro) -> Option<Macro> {
		self.macros.insert(key, value)
	}

	pub fn remove_macro(&mut self, key: &str) -> Option<(String, Macro)> {
		self.macros.remove_entry(key)
	}
}
