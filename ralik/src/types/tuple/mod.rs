use super::{GenericTypeBuilder, TypeKind};
use crate::context::Context;

pub fn make_tuple_name(element_types: impl Iterator<Item = impl AsRef<str>>) -> String {
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

pub fn tuple_generic(_context: &Context, element_types: &[&str]) -> Result<GenericTypeBuilder, anyhow::Error> {
	Ok(GenericTypeBuilder {
		kind: TypeKind::Tuple,
		type_parameters: element_types
			.iter()
			.map(|element_type| (*element_type).into())
			.collect(),
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: Default::default(),
	})
}
