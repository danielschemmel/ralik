use std::collections::HashMap;
use std::sync::Arc;

use super::{MemberFunction, Type};

pub(crate) type TupleFunctionStore = HashMap<String, MemberFunction>;

pub(crate) struct TupleType {
	name: String,
	element_types: Vec<Arc<dyn Type>>,
	functions: TupleFunctionStore,
}

impl TupleType {
	pub fn new(name: impl Into<String>, element_types: impl Into<Vec<Arc<dyn Type>>>) -> Self {
		Self {
			name: name.into(),
			element_types: element_types.into(),
			functions: TupleFunctionStore::new(),
		}
	}
}

impl Type for TupleType {
	fn name(&self) -> &str {
		&self.name
	}

	fn get_function(&self, key: &str) -> Option<&MemberFunction> {
		self.functions.get(key)
	}

	fn insert_function(&mut self, key: String, value: MemberFunction) -> Option<MemberFunction> {
		self.functions.insert(key, value)
	}

	fn remove_function(&mut self, key: &str) -> Option<(String, MemberFunction)> {
		self.functions.remove_entry(key)
	}
}

impl std::fmt::Debug for TupleType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Type")
			.field("name", &self.name())
			.field("sub_types", &self.element_types)
			.field("functions", &super::debug::FunctionNameListFormatter(&self.functions))
			.finish()
	}
}