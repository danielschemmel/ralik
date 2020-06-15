use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeKind, Variant};

pub type EnumType = BasicType<EnumImpl>;

pub struct EnumImpl {
	name: String,
	variants: HashMap<String, Variant>,
}

impl EnumType {
	pub fn new(name: impl Into<String>, variants: impl Iterator<Item = (impl Into<String>, Variant)>) -> Self {
		Self::from_base(EnumImpl {
			name: name.into(),
			variants: variants.map(|(name, r#type)| (name.into(), r#type)).collect(),
		})
	}

	pub fn new_unit(name: impl Into<String>) -> Self {
		Self::from_base(EnumImpl {
			name: name.into(),
			variants: HashMap::new(),
		})
	}
}

impl BasicTypeBase for EnumImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Enum
	}

	fn variants(&self) -> Option<&HashMap<String, Variant>> {
		Some(&self.variants)
	}
}
