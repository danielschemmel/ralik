use crate::Value;

use super::Context;

impl Context {
	pub fn get_variable(&self, key: &str) -> Option<Value> {
		self.0.variables.read().unwrap().get(key).cloned()
	}

	pub fn insert_variable(&self, key: impl Into<String>, value: impl Into<Value>) -> Option<Value> {
		self.0.variables.write().unwrap().insert(key.into(), value.into())
	}

	pub fn remove_variable(&self, key: &str) -> Option<(String, Value)> {
		self.0.variables.write().unwrap().remove_entry(key)
	}
}
