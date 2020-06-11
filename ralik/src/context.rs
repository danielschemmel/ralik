use std::collections::HashMap;
use std::ops::Index;

use super::Value;

pub struct Context {
	map: HashMap<String, Value>,
}

impl Context {
	pub fn new() -> Self {
		Context { map: HashMap::new() }
	}

	pub fn get(&self, key: &str) -> Option<&Value> {
		self.map.get(key)
	}

	pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
		self.map.get_mut(key)
	}

	pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
		self.map.insert(key, value)
	}

	pub fn remove(&mut self, key: &str) -> Option<(String, Value)> {
		self.map.remove_entry(key)
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

impl Index<&str> for Context {
	type Output = Value;

	fn index(&self, index: &str) -> &Self::Output {
		&self.map[index]
	}
}
