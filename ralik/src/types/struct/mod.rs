use std::collections::HashMap;

use super::{MemberFunction, Type, TypeHandle, TypeKind};

pub type StructFunctionStore = HashMap<String, MemberFunction>;

pub struct StructType {
	name: String,
	fields: HashMap<String, TypeHandle>,
	functions: StructFunctionStore,
}

impl StructType {
	pub fn new(name: impl Into<String>, fields: impl Iterator<Item = (String, TypeHandle)>) -> Self {
		Self {
			name: name.into(),
			fields: fields.collect(),
			functions: StructFunctionStore::new(),
		}
	}
}

impl Type for StructType {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Struct
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

impl std::fmt::Debug for StructType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Type")
			.field("name", &self.name())
			.field("fields", &self.fields)
			.field("functions", &super::debug::FunctionNameListFormatter(&self.functions))
			.finish()
	}
}
