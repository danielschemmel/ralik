use super::{BasicType, BasicTypeBase, TypeKind};

mod functions;
mod ops;

pub type CharType = BasicType<CharImpl>;
pub const fn name() -> &'static str {
	"char"
}

pub struct CharImpl;

impl CharType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for CharType {
	fn default() -> Self {
		BasicType::from_base_with_functions(
			CharImpl,
			vec![
				(crate::ops::EQUAL, ops::equal),
				(crate::ops::NOT_EQUAL, ops::not_equal),
				(crate::ops::LESS, ops::less),
				(crate::ops::LESS_OR_EQUAL, ops::less_or_equal),
				(crate::ops::GREATER, ops::greater),
				(crate::ops::GREATER_OR_EQUAL, ops::greater_or_equal),
				("clone", functions::clone),
				("eq_ignore_ascii_case", functions::eq_ignore_ascii_case),
				("is_alphabetic", functions::is_alphabetic),
				("is_alphanumeric", functions::is_alphanumeric),
				("is_ascii", functions::is_ascii),
				("is_ascii_alphabetic", functions::is_ascii_alphabetic),
				("is_ascii_alphanumeric", functions::is_ascii_alphanumeric),
				("is_ascii_control", functions::is_ascii_control),
				("is_ascii_digit", functions::is_ascii_digit),
				("is_ascii_graphic", functions::is_ascii_graphic),
				("is_ascii_hexdigit", functions::is_ascii_hexdigit),
				("is_ascii_lowercase", functions::is_ascii_lowercase),
				("is_ascii_punctuation", functions::is_ascii_punctuation),
				("is_ascii_uppercase", functions::is_ascii_uppercase),
				("is_ascii_whitespace", functions::is_ascii_whitespace),
				("is_control", functions::is_control),
				("is_digit", functions::is_digit),
				("is_lowercase", functions::is_lowercase),
				("is_numeric", functions::is_numeric),
				("is_uppercase", functions::is_uppercase),
				("is_whitespace", functions::is_whitespace),
				("len_utf16", functions::len_utf16),
				("len_utf8", functions::len_utf8),
				("to_ascii_lowercase", functions::to_ascii_lowercase),
				("to_ascii_uppercase", functions::to_ascii_uppercase),
				("to_string", functions::to_string),
			],
		)
	}
}

impl BasicTypeBase for CharImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Char
	}
}
