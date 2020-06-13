use std::collections::HashMap;

use super::eval::{Overflow};
use crate::{Context, CallError, Value};

mod arguments;

mod bool;
pub use self::bool::BoolType;
pub use self::bool::NAME as BoolName;

mod char;
pub use self::char::CharType;
pub use self::bool::NAME as CharName;

mod integer;
pub use self::integer::IntegerType;
pub use self::bool::NAME as IntegerName;

mod string;
pub use self::string::StringType;
pub use self::bool::NAME as StringName;

//mod array;
//mod option;

pub type MemberFunction = fn(&Context, &[Value]) -> Result<Value, CallError>;

pub trait Type : std::fmt::Debug {
	fn name(&self) -> &str;

	fn get_function(&self, key: &str) -> Option<&MemberFunction>;
	fn get_function_mut(&mut self, key: &str) -> Option<&mut MemberFunction>;
	fn insert_function(&mut self, key: String, value: MemberFunction) -> Option<MemberFunction>;
	fn remove_function(&mut self, key: &str) -> Option<(String, MemberFunction)>;
}

type FunctionStore = HashMap<String, MemberFunction>;

pub trait BasicTypeBase {
	fn name(&self) -> &str;
	fn register_functions(&self, functions: &mut FunctionStore);
}

pub struct BasicType<T: BasicTypeBase> {
	base: T,
	functions: FunctionStore,
}

impl<T: BasicTypeBase> BasicType<T> {
	pub fn from_base(base: T) -> Self {
		let mut functions = FunctionStore::new();
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

	fn get_function_mut(&mut self, key: &str) -> Option<&mut MemberFunction> {
		self.functions.get_mut(key)
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
			.field(
				"functions",
				&FunctionNameListFormatter {
					functions: &self.functions,
				},
			)
			.finish()
	}
}

struct FunctionNameListFormatter<'a> {
	functions: &'a HashMap<String, MemberFunction>,
}
impl<'a> std::fmt::Debug for FunctionNameListFormatter<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.functions.keys()).finish()
	}
}
