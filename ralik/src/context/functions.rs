use super::{Context, Function};

impl Context {
	pub fn get_function(&self, key: &str) -> Option<Function> {
		self.0.functions.read().unwrap().get(key).cloned()
	}

	pub fn insert_function(&self, key: impl Into<String>, value: Function) -> Option<Function> {
		self.0.functions.write().unwrap().insert(key.into(), value)
	}

	pub fn remove_function(&self, key: &str) -> Option<(String, Function)> {
		self.0.functions.write().unwrap().remove_entry(key)
	}
}
