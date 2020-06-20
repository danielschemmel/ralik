use anyhow::ensure;

use super::{GenericTypeBuilder, MemberFunction, TypeKind};
use crate::context::Context;

mod functions;
mod ops;

pub fn make_array_name(element_type: &str) -> String {
	format!("[{}]", element_type)
}

pub fn array_generic(_context: &Context, element_type: &[&str]) -> Result<GenericTypeBuilder, anyhow::Error> {
	ensure!(
		element_type.len() == 1,
		"Can only create arrays with exactly one element type ({} provided)",
		element_type.len()
	);
	let element_type = element_type[0];

	Ok(GenericTypeBuilder {
		kind: TypeKind::Array,
		type_parameters: vec![element_type.into()],
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: [
			(crate::ops::INDEX, ops::index as MemberFunction),
			("clone", functions::clone),
			("is_empty", functions::is_empty),
			("len", functions::len),
		]
		.iter()
		.map(|(name, function)| ((*name).to_owned(), *function))
		.collect(),
	})
}
