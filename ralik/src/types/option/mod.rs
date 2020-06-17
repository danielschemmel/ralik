use lazy_static::lazy_static;

use std::collections::HashMap;

use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind, Variant};

mod functions;
mod ops;

pub(crate) type OptionType = BasicType<OptionImpl>;

pub(crate) struct OptionImpl {
	name: String,
	element_type: [TypeHandle; 1],
	variants: [Variant; 2],
}

pub fn make_option_name(element_type: &str) -> String {
	format!("std::option::Option<{}>", element_type)
}

lazy_static! {
	static ref VARIANT_NAMES: HashMap<Box<str>, usize> =
		vec![("None".into(), 0), ("Some".into(), 1)].into_iter().collect();
}

impl OptionType {
	pub fn new(name: impl Into<String>, element_type: TypeHandle) -> Self {
		Self::from_base_with_functions(
			OptionImpl {
				name: name.into(),
				element_type: [element_type.clone()],
				variants: [
					Variant::Unit("None".into()),
					Variant::Tuple("Some".into(), Box::new([element_type.clone()])),
				],
			},
			vec![
				(crate::ops::UNWRAP, ops::unwrap),
				("unwrap", ops::unwrap),
				("is_none", functions::is_none),
				("is_some", functions::is_some),
			],
		)
	}
}

impl BasicTypeBase for OptionImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Enum
	}

	fn type_parameters(&self) -> &[TypeHandle] {
		&self.element_type
	}

	fn variants(&self) -> Option<(&HashMap<Box<str>, usize>, &[Variant])> {
		Some((&VARIANT_NAMES, &self.variants))
	}
}
