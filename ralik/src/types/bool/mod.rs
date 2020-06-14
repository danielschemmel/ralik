use super::{BasicFunctionStore, BasicType, BasicTypeBase, TypeKind};

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
		BasicType::from_base(BoolImpl)
	}
}

impl BasicTypeBase for BoolImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Bool
	}

	fn register_functions(&self, functions: &mut BasicFunctionStore) {
		functions.insert(crate::ops::NOT.into(), ops::not);
		functions.insert(crate::ops::BIT_AND.into(), ops::bit_and);
		functions.insert(crate::ops::BIT_OR.into(), ops::bit_or);
		functions.insert(crate::ops::BIT_XOR.into(), ops::bit_xor);
		functions.insert(crate::ops::EQUAL.into(), ops::equal);
		functions.insert(crate::ops::NOT_EQUAL.into(), ops::not_equal);
		functions.insert(crate::ops::LESS.into(), ops::less);
		functions.insert(crate::ops::LESS_OR_EQUAL.into(), ops::less_or_equal);
		functions.insert(crate::ops::GREATER.into(), ops::greater);
		functions.insert(crate::ops::GREATER_OR_EQUAL.into(), ops::greater_or_equal);

		functions.insert("to_string".into(), functions::to_string);
	}
}
