use super::{BasicFunctionStore, BasicType, BasicTypeBase};

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

	fn register_functions(&self, functions: &mut BasicFunctionStore) {
		functions.insert(crate::ops::EQUAL.into(), ops::equal);
		functions.insert(crate::ops::NOT_EQUAL.into(), ops::not_equal);
		functions.insert(crate::ops::LESS.into(), ops::less);
		functions.insert(crate::ops::LESS_OR_EQUAL.into(), ops::less_or_equal);
		functions.insert(crate::ops::GREATER.into(), ops::greater);
		functions.insert(crate::ops::GREATER_OR_EQUAL.into(), ops::greater_or_equal);

		functions.insert("eq_ignore_ascii_case".into(), functions::eq_ignore_ascii_case);
		functions.insert("is_alphabetic".into(), functions::is_alphabetic);
		functions.insert("is_alphanumeric".into(), functions::is_alphanumeric);
		functions.insert("is_ascii".into(), functions::is_ascii);
		functions.insert("is_ascii_alphabetic".into(), functions::is_ascii_alphabetic);
		functions.insert("is_ascii_alphanumeric".into(), functions::is_ascii_alphanumeric);
		functions.insert("is_ascii_control".into(), functions::is_ascii_control);
		functions.insert("is_ascii_digit".into(), functions::is_ascii_digit);
		functions.insert("is_ascii_graphic".into(), functions::is_ascii_graphic);
		functions.insert("is_ascii_hexdigit".into(), functions::is_ascii_hexdigit);
		functions.insert("is_ascii_lowercase".into(), functions::is_ascii_lowercase);
		functions.insert("is_ascii_punctuation".into(), functions::is_ascii_punctuation);
		functions.insert("is_ascii_uppercase".into(), functions::is_ascii_uppercase);
		functions.insert("is_ascii_whitespace".into(), functions::is_ascii_whitespace);
		functions.insert("is_control".into(), functions::is_control);
		functions.insert("is_digit".into(), functions::is_digit);
		functions.insert("is_lowercase".into(), functions::is_lowercase);
		functions.insert("is_numeric".into(), functions::is_numeric);
		functions.insert("is_uppercase".into(), functions::is_uppercase);
		functions.insert("is_whitespace".into(), functions::is_whitespace);
		functions.insert("len_utf16".into(), functions::len_utf16);
		functions.insert("len_utf8".into(), functions::len_utf8);
		functions.insert("to_ascii_lowercase".into(), functions::to_ascii_lowercase);
		functions.insert("to_ascii_uppercase".into(), functions::to_ascii_uppercase);
		functions.insert("to_string".into(), functions::to_string);
	}
}
