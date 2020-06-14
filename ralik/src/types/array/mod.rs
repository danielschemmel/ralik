use std::collections::HashMap;
use std::sync::Arc;

use super::{MemberFunction, Type};

mod functions;
mod ops;

pub(crate) type ArrayFunctionStore = HashMap<String, MemberFunction>;

pub(crate) struct ArrayType {
	name: String,
	element_type: Arc<dyn Type>,
	functions: ArrayFunctionStore,
}

pub fn name(element_type: &str) -> String {
	format!("[{}]", element_type)
}

impl ArrayType {
	pub fn new(name: impl Into<String>, element_type: Arc<dyn Type>) -> Arc<Self> {
		let mut 		functions= ArrayFunctionStore::new();

		functions.insert(crate::ops::INDEX.into(), ops::index);

		functions.insert("is_empty".into(), functions::is_empty);
		functions.insert("len".into(), functions::len);

		let result = Arc::new(Self {
			name: name.into(),
			element_type,
			functions,
		});

		result
	}
}

impl Type for ArrayType {
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

impl std::fmt::Debug for ArrayType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Type")
			.field("name", &self.name())
			.field("element_type", &self.element_type)
			.field("functions", &super::debug::FunctionNameListFormatter(&self.functions))
			.finish()
	}
}
