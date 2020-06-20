use super::{MemberFunction, TypeBuilder, TypeKind};

mod functions;
mod ops;

pub const fn make_bool_name() -> &'static str {
	"bool"
}

pub fn new_bool_type() -> TypeBuilder {
	TypeBuilder {
		name: make_bool_name().into(),
		kind: TypeKind::Bool,
		type_parameters: Default::default(),
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: [
			(crate::ops::NOT, ops::not as MemberFunction),
			(crate::ops::BIT_AND, ops::bit_and),
			(crate::ops::BIT_OR, ops::bit_or),
			(crate::ops::BIT_XOR, ops::bit_xor),
			(crate::ops::EQUAL, ops::equal),
			(crate::ops::NOT_EQUAL, ops::not_equal),
			(crate::ops::LESS, ops::less),
			(crate::ops::LESS_OR_EQUAL, ops::less_or_equal),
			(crate::ops::GREATER, ops::greater),
			(crate::ops::GREATER_OR_EQUAL, ops::greater_or_equal),
			("clone", functions::clone),
			("to_string", functions::to_string),
		]
		.iter()
		.map(|(name, function)| ((*name).to_owned(), *function))
		.collect(),
	}
}
