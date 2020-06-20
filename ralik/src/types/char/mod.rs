use super::{MemberFunction, TypeBuilder, TypeKind};

mod functions;
mod ops;

pub const fn make_char_name() -> &'static str {
	"char"
}

pub fn new_char_type() -> TypeBuilder {
	TypeBuilder {
		name: make_char_name().into(),
		kind: TypeKind::Char,
		type_parameters: Default::default(),
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: [
			(crate::ops::EQUAL, ops::equal as MemberFunction),
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
		]
		.iter()
		.map(|(name, function)| ((*name).to_owned(), *function))
		.collect(),
	}
}
