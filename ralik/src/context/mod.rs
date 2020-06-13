use std::collections::HashMap;
use std::sync::Arc;

use crate::{Type, CallError, Value, MissingBoolType, MissingCharType, MissingIntegerType, MissingStringType};

mod debug;

pub type Function = fn(&[Value]) -> Result<Value, CallError>;
pub type Macro = fn(&[Value]) -> Result<Value, CallError>;

#[derive(Clone)]
pub struct Context {
	types: HashMap<String, Arc<dyn Type>>,
	variables: HashMap<String, Value>,
	functions: HashMap<String, Function>,
	macros: HashMap<String, Macro>,
}

// Common
impl Context {
	pub fn new() -> Self {
		let mut context = Self::new_empty();

		context.insert_type(Arc::new(crate::types::BoolType::new()));
		context.insert_type(Arc::new(crate::types::CharType::new()));
		context.insert_type(Arc::new(crate::types::IntegerType::new()));
		context.insert_type(Arc::new(crate::types::StringType::new()));

		context
	}

	pub fn new_empty() -> Self {
		Context {
			types: HashMap::new(),
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

// Types
impl Context {
	pub fn get_type(&self, key: &str) -> Option<&Arc<dyn Type>> {
		self.types.get(key)
	}

	pub fn get_bool_type(&self) -> Result<&Arc<dyn Type>, MissingBoolType> {
		self.types.get(crate::types::BoolName).ok_or_else(|| MissingBoolType)
	}

	pub fn get_char_type(&self) -> Result<&Arc<dyn Type>, MissingCharType> {
		self.types.get(crate::types::CharName).ok_or_else(|| MissingCharType)
	}

	pub fn get_integer_type(&self) -> Result<&Arc<dyn Type>, MissingIntegerType> {
		self.types.get(crate::types::IntegerName).ok_or_else(|| MissingIntegerType)
	}

	pub fn get_string_type(&self) -> Result<&Arc<dyn Type>, MissingStringType> {
		self.types.get(crate::types::StringName).ok_or_else(|| MissingStringType)
	}

	pub fn get_type_mut(&mut self, key: &str) -> Option<&mut Arc<dyn Type>> {
		self.types.get_mut(key)
	}

	pub fn insert_type(&mut self, value: Arc<dyn Type>) -> Option<Arc<dyn Type>> {
		let name = value.name().to_string();
		self.types.insert(name, value)
	}

	pub fn remove_type(&mut self, key: &str) -> Option<(String, Arc<dyn Type>)> {
		let (key, value) = self.types.remove_entry(key)?;
		if Arc::strong_count(&value) == 1 {
			Some((key, value))
		} else {
			// There are still values using this type, it cannot be removed now
			assert!(self.types.insert(key, value).is_none());
			None
		}
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

	pub fn insert_variable<K: Into<String>, V: Into<Value>>(&mut self, key: K, value: V) -> Option<Value> {
		self.variables.insert(key.into(), value.into())
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

	pub fn insert_function<K: Into<String>>(&mut self, key: K, value: Function) -> Option<Function> {
		self.functions.insert(key.into(), value)
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

	pub fn insert_macro<K: Into<String>>(&mut self, key: K, value: Macro) -> Option<Macro> {
		self.macros.insert(key.into(), value)
	}

	pub fn remove_macro(&mut self, key: &str) -> Option<(String, Macro)> {
		self.macros.remove_entry(key)
	}
}
