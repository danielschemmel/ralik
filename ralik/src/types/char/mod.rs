use super::{BasicType, BasicTypeBase, FunctionStore};

mod functions;
mod ops;

pub type CharType = BasicType<CharImpl>;
pub const NAME: &str = "char";

pub struct CharImpl;

impl CharType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for CharType {
	fn default() -> Self {
		BasicType::from_base(CharImpl)
	}
}

impl BasicTypeBase for CharImpl {
	fn name(&self) -> &str {
		NAME
	}

	fn register_functions(&self, functions: &mut FunctionStore) {
		functions.insert(crate::ops::EQUAL.into(), ops::equal);
		functions.insert(crate::ops::NOT_EQUAL.into(), ops::not_equal);
		functions.insert(crate::ops::LESS.into(), ops::less);
		functions.insert(crate::ops::LESS_OR_EQUAL.into(), ops::less_or_equal);
		functions.insert(crate::ops::GREATER.into(), ops::greater);
		functions.insert(crate::ops::GREATER_OR_EQUAL.into(), ops::greater_or_equal);

		functions.insert("to_string".into(), functions::to_string);
	}
}
