use super::{BasicType, BasicTypeBase, TypeKind};

mod functions;
mod ops;

pub type UnitStructType = BasicType<UnitStructImpl>;

pub struct UnitStructImpl {
	name: Box<str>,
}

impl UnitStructType {
	pub fn new(name: impl Into<Box<str>>) -> Self {
		let name = name.into();

		BasicType::from_base_with_functions(
			UnitStructImpl { name },
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

impl BasicTypeBase for UnitStructImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::UnitStruct
	}
}
