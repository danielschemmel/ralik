use super::{BasicFunctionStore, BasicType, BasicTypeBase, TypeKind};

mod ops;

pub type UnitType = BasicType<UnitImpl>;
pub const fn name() -> &'static str {
	"()"
}

pub struct UnitImpl;

impl UnitType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for UnitType {
	fn default() -> Self {
		BasicType::from_base(UnitImpl)
	}
}

impl BasicTypeBase for UnitImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Unit
	}

	fn register_functions(&self, functions: &mut BasicFunctionStore) {
		functions.insert(crate::ops::EQUAL.into(), ops::equal);
		functions.insert(crate::ops::NOT_EQUAL.into(), ops::not_equal);
		functions.insert(crate::ops::LESS.into(), ops::less);
		functions.insert(crate::ops::LESS_OR_EQUAL.into(), ops::less_or_equal);
		functions.insert(crate::ops::GREATER.into(), ops::greater);
		functions.insert(crate::ops::GREATER_OR_EQUAL.into(), ops::greater_or_equal);
	}
}
