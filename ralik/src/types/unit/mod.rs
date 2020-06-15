use super::{BasicType, BasicTypeBase, TypeKind};

mod functions;
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
		BasicType::from_base_with_functions(
			UnitImpl,
			vec![
				(crate::ops::EQUAL, ops::equal),
				(crate::ops::NOT_EQUAL, ops::not_equal),
				(crate::ops::LESS, ops::less),
				(crate::ops::LESS_OR_EQUAL, ops::less_or_equal),
				(crate::ops::GREATER, ops::greater),
				(crate::ops::GREATER_OR_EQUAL, ops::greater_or_equal),
				("clone", functions::clone),
			],
		)
	}
}

impl BasicTypeBase for UnitImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Unit
	}
}
