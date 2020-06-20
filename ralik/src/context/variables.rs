use std::collections::hash_map::Entry;

use crate::Value;

use super::{Context, Thing};

impl Context {
	pub fn get_variable(&self, key: &str) -> Option<Value> {
		match self.0.names.read().unwrap().get(key) {
			Some(Thing::Variable(value)) => Some(value.clone()),
			_ => None,
		}
	}

	pub fn insert_variable(&self, key: impl Into<String>, value: impl Into<Value>) {
		let mut names = self.0.names.write().unwrap();
		match names.entry(key.into()) {
			Entry::Occupied(entry) => panic!("The name `{}` is defined multiple times", entry.key()),
			Entry::Vacant(entry) => {
				entry.insert(Thing::Variable(value.into()));
			}
		}
	}
}
