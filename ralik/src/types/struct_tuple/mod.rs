use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

pub type StructTupleType = BasicType<StructTupleImpl>;

pub struct StructTupleImpl {
	name: String,
	element_types: Vec<TypeHandle>,
}

impl StructTupleType {
	pub fn new(name: impl Into<String>, element_types: impl Into<Vec<TypeHandle>>) -> Self {
		Self::from_base(StructTupleImpl {
			name: name.into(),
			element_types: element_types.into(),
		})
	}
}

impl BasicTypeBase for StructTupleImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Struct
	}

	fn type_parameters(&self) -> &[TypeHandle] {
		&self.element_types
	}
}
