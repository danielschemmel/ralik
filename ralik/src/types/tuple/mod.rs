use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

pub type TupleType = BasicType<TupleImpl>;

pub struct TupleImpl {
	name: Box<str>,
	element_types: Box<[TypeHandle]>,
}

pub fn make_name(element_types: impl Iterator<Item = impl AsRef<str>>) -> String {
	let mut name = "(".to_owned();
	for (i, element_type_name) in element_types.enumerate() {
		if i > 0 {
			name.push_str(", ");
		}
		name.push_str(element_type_name.as_ref());
	}
	name.push_str(")");

	name.into()
}

impl TupleType {
	pub fn new(name: impl Into<Box<str>>, element_types: impl Into<Box<[TypeHandle]>>) -> Self {
		let (name, element_types) = (name.into(), element_types.into());

		Self::from_base(TupleImpl { name, element_types })
	}

	pub fn new_unit(name: impl Into<Box<str>>) -> Self {
		Self::from_base(TupleImpl {
			name: name.into(),
			element_types: Box::new([]),
		})
	}
}

impl BasicTypeBase for TupleImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Tuple
	}

	fn type_parameters(&self) -> &[TypeHandle] {
		&self.element_types
	}

	fn fields(&self) -> (Option<&HashMap<Box<str>, usize>>, &[TypeHandle]) {
		(None, &self.element_types)
	}
}
