use std::collections::HashMap;

use super::{MemberFunction, Type};

pub type BasicFunctionStore = HashMap<String, MemberFunction>;

pub trait BasicTypeBase {
	fn name(&self) -> &str;
	fn register_functions(&self, functions: &mut BasicFunctionStore);
}

pub struct BasicType<T: BasicTypeBase> {
	base: T,
	functions: BasicFunctionStore,
}

impl<T: BasicTypeBase> BasicType<T> {
	pub fn from_base(base: T) -> Self {
		let mut functions = BasicFunctionStore::new();
		base.register_functions(&mut functions);
		Self { base, functions }
	}
}

impl<T: BasicTypeBase> Type for BasicType<T> {
	fn name(&self) -> &str {
		self.base.name()
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

impl<T: BasicTypeBase> std::fmt::Debug for BasicType<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Type")
			.field("name", &self.name())
			.field("functions", &super::debug::FunctionNameListFormatter(&self.functions))
			.finish()
	}
}
