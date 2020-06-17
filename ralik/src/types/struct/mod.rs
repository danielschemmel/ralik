use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

pub type StructType = BasicType<StructImpl>;

pub struct StructImpl {
	name: Box<str>,
	field_names: HashMap<Box<str>, usize>,
	field_types: Vec<TypeHandle>,
}

impl StructType {
	pub fn new(name: impl Into<Box<str>>, fields: impl Iterator<Item = (impl Into<Box<str>>, TypeHandle)>) -> Self {
		let fields = fields
			.enumerate()
			.map(|(i, (name, r#type))| ((name.into(), i), r#type))
			.unzip();

		Self::from_base(StructImpl {
			name: name.into(),
			field_names: fields.0,
			field_types: fields.1,
		})
	}

	pub fn new_empty(name: impl Into<Box<str>>) -> Self {
		Self::from_base(StructImpl {
			name: name.into(),
			field_names: HashMap::new(),
			field_types: Vec::new(),
		})
	}
}

impl BasicTypeBase for StructImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Struct
	}

	fn fields(&self) -> (Option<&HashMap<Box<str>, usize>>, &[TypeHandle]) {
		(Some(&self.field_names), &self.field_types)
	}
}
