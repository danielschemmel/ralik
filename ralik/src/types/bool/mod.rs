use super::{BasicType, BasicTypeBase, TypeKind};

mod functions;
mod ops;

pub type BoolType = BasicType<BoolImpl>;
pub const fn name() -> &'static str {
	"bool"
}

pub struct BoolImpl;

impl BoolType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for BoolType {
	fn default() -> Self {
		BasicType::from_base_with_functions(
			BoolImpl,
			vec![
				(crate::ops::NOT, ops::not),
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
			],
		)
	}
}

impl BasicTypeBase for BoolImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Bool
	}
}
