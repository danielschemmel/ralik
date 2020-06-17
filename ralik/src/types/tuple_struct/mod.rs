use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

pub type TupleStructType = BasicType<TupleStructImpl>;

pub struct TupleStructImpl {
	name: Box<str>,
	element_types: Vec<TypeHandle>,
}

impl TupleStructType {
	pub fn new(name: impl Into<Box<str>>, element_types: impl Into<Vec<TypeHandle>>) -> Self {
		let (name, element_types) = (name.into(), element_types.into());

		Self::from_base(TupleStructImpl { name, element_types })
	}
}

impl BasicTypeBase for TupleStructImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::TupleStruct
	}

	fn fields(&self) -> (Option<&HashMap<Box<str>, usize>>, &[TypeHandle]) {
		(None, &self.element_types)
	}
}
