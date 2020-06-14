use super::{BasicFunctionStore, BasicType, BasicTypeBase};

mod functions;
mod ops;

pub type StringType = BasicType<StringImpl>;
pub const fn name() -> &'static str {
	"String"
}

pub struct StringImpl;

impl StringType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for StringType {
	fn default() -> Self {
		BasicType::from_base(StringImpl)
	}
}

impl BasicTypeBase for StringImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn register_functions(&self, functions: &mut BasicFunctionStore) {
		functions.insert(crate::ops::ADD.into(), ops::add);
		functions.insert(crate::ops::EQUAL.into(), ops::equal);
		functions.insert(crate::ops::NOT_EQUAL.into(), ops::not_equal);
		functions.insert(crate::ops::LESS.into(), ops::less);
		functions.insert(crate::ops::LESS_OR_EQUAL.into(), ops::less_or_equal);
		functions.insert(crate::ops::GREATER.into(), ops::greater);
		functions.insert(crate::ops::GREATER_OR_EQUAL.into(), ops::greater_or_equal);

		functions.insert("eq_ignore_ascii_case".into(), functions::eq_ignore_ascii_case);
		functions.insert("is_ascii".into(), functions::is_ascii);
		functions.insert("is_char_boundary".into(), functions::is_char_boundary);
		functions.insert("is_empty".into(), functions::is_empty);
		functions.insert("len".into(), functions::len);
		functions.insert("repeat".into(), functions::repeat);
		functions.insert("to_ascii_lowercase".into(), functions::to_ascii_lowercase);
		functions.insert("to_ascii_uppercase".into(), functions::to_ascii_uppercase);
		functions.insert("to_lowercase".into(), functions::to_lowercase);
		functions.insert("to_string".into(), functions::to_string);
		functions.insert("to_uppercase".into(), functions::to_uppercase);
		functions.insert("trim".into(), functions::trim);
		functions.insert("trim_end".into(), functions::trim_end);
		functions.insert("trim_start".into(), functions::trim_start);
	}
}
