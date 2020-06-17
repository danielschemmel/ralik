use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeKind, Variant};

pub type EnumType = BasicType<EnumImpl>;

pub struct EnumImpl {
	name: Box<str>,
	variant_names: HashMap<Box<str>, usize>,
	variant_kinds: Vec<Variant>,
}

impl EnumType {
	pub fn new(name: impl Into<Box<str>>, variants: impl Iterator<Item = Variant>) -> Self {
		let variants = variants
			.enumerate()
			.map(|(i, variant)| ((variant.name().into(), i), variant))
			.unzip();

		Self::from_base(EnumImpl {
			name: name.into(),
			variant_names: variants.0,
			variant_kinds: variants.1,
		})
	}

	pub fn new_unit(name: impl Into<Box<str>>) -> Self {
		Self::from_base(EnumImpl {
			name: name.into(),
			variant_names: HashMap::new(),
			variant_kinds: Vec::new(),
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

	fn variants(&self) -> Option<(&HashMap<Box<str>, usize>, &[Variant])> {
		Some((&self.variant_names, &self.variant_kinds))
	}
}
