use super::{Context, Macro};

impl Context {
	pub fn get_macro(&self, key: &str) -> Option<Macro> {
		self.0.macros.read().unwrap().get(key).cloned()
	}

	pub fn insert_macro(&self, key: impl Into<String>, value: Macro) -> Option<Macro> {
		self.0.macros.write().unwrap().insert(key.into(), value)
	}

	pub fn remove_macro(&self, key: &str) -> Option<(String, Macro)> {
		self.0.macros.write().unwrap().remove_entry(key)
	}
}
