use super::{BasicType, BasicTypeBase, TypeKind};

mod functions;
mod ops;

pub type StringType = BasicType<StringImpl>;
pub const fn name() -> &'static str {
	"std::string::String"
}

pub struct StringImpl;

impl StringType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for StringType {
	fn default() -> Self {
		BasicType::from_base_with_functions(
			StringImpl,
			vec![
				(crate::ops::ADD, ops::add),
				(crate::ops::EQUAL, ops::equal),
				(crate::ops::NOT_EQUAL, ops::not_equal),
				(crate::ops::LESS, ops::less),
				(crate::ops::LESS_OR_EQUAL, ops::less_or_equal),
				(crate::ops::GREATER, ops::greater),
				(crate::ops::GREATER_OR_EQUAL, ops::greater_or_equal),
				("clone", functions::clone),
				("eq_ignore_ascii_case", functions::eq_ignore_ascii_case),
				("is_ascii", functions::is_ascii),
				("is_char_boundary", functions::is_char_boundary),
				("is_empty", functions::is_empty),
				("len", functions::len),
				("repeat", functions::repeat),
				("to_ascii_lowercase", functions::to_ascii_lowercase),
				("to_ascii_uppercase", functions::to_ascii_uppercase),
				("to_lowercase", functions::to_lowercase),
				("to_string", functions::to_string),
				("to_uppercase", functions::to_uppercase),
				("trim", functions::trim),
				("trim_end", functions::trim_end),
				("trim_start", functions::trim_start),
			],
		)
	}
}

impl BasicTypeBase for StringImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::String
	}
}
