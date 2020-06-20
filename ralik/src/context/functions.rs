use std::collections::hash_map::Entry;

use super::{Context, Function, Thing};

impl Context {
	pub fn get_function(&self, key: &str) -> Option<Function> {
		match self.0.names.read().unwrap().get(key) {
			Some(Thing::Function(value)) => Some(value.clone()),
			_ => None,
		}
	}

	pub fn insert_function(&self, key: impl Into<String>, value: Function) {
		let mut names = self.0.names.write().unwrap();
		match names.entry(key.into()) {
			Entry::Occupied(entry) => panic!("The name `{}` is defined multiple times", entry.key()),
			Entry::Vacant(entry) => {
				entry.insert(Thing::Function(value));
			}
		}
	}
}
