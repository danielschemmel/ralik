use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

pub type StructType = BasicType<StructImpl>;

pub struct StructImpl {
	name: String,
	fields: HashMap<String, TypeHandle>,
}

impl StructType {
	pub fn new(name: impl Into<String>, fields: impl Iterator<Item = (impl Into<String>, TypeHandle)>) -> Self {
		Self::from_base(StructImpl {
			name: name.into(),
			fields: fields.map(|(name, r#type)| (name.into(), r#type)).collect(),
		})
	}

	pub fn new_unit(name: impl Into<String>) -> Self {
		Self::from_base(StructImpl {
			name: name.into(),
			fields: HashMap::new(),
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

	fn fields(&self) -> Option<&HashMap<String, TypeHandle>> {
		Some(&self.fields)
	}
}
