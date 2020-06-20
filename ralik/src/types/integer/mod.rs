use super::{MemberFunction, TypeBuilder, TypeKind};

mod functions;
mod ops;

pub const fn make_integer_name() -> &'static str {
	"Integer"
}

pub fn new_integer_type() -> TypeBuilder {
	TypeBuilder {
		name: make_integer_name().into(),
		kind: TypeKind::Integer,
		type_parameters: Default::default(),
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: [
			(crate::ops::NOT, ops::not as MemberFunction),
			(crate::ops::NEGATE, ops::negate),
			(crate::ops::NOT, ops::not),
			(crate::ops::MUL, ops::multiply),
			(crate::ops::DIV, ops::divide),
			(crate::ops::REM, ops::remainder),
			(crate::ops::ADD, ops::add),
			(crate::ops::SUB, ops::subtract),
			(crate::ops::SHL, ops::shift_left),
			(crate::ops::SHR, ops::shift_right),
			(crate::ops::BIT_AND, ops::bit_and),
			(crate::ops::BIT_OR, ops::bit_or),
			(crate::ops::BIT_XOR, ops::bit_xor),
			(crate::ops::EQUAL, ops::equal),
			(crate::ops::NOT_EQUAL, ops::not_equal),
			(crate::ops::LESS, ops::less),
			(crate::ops::LESS_OR_EQUAL, ops::less_or_equal),
			(crate::ops::GREATER, ops::greater),
			(crate::ops::GREATER_OR_EQUAL, ops::greater_or_equal),
			("abs", functions::abs),
			//("checked_div", functions::checked_div),
			("clone", functions::clone),
			("is_negative", functions::is_negative),
			("is_positive", functions::is_positive),
			("pow", functions::pow),
			("signum", functions::signum),
			("to_string", functions::to_string),
		]
		.iter()
		.map(|(name, function)| ((*name).to_owned(), *function))
		.collect(),
	}
}
