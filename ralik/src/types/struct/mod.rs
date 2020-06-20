use super::{TypeBuilder, TypeKind};

//mod functions;
//mod ops;

pub fn new_struct_type(
	name: impl Into<String>,
	fields: impl Iterator<Item = (impl Into<String>, impl Into<String>)>,
) -> TypeBuilder {
	let (field_names, field_types) = fields
		.enumerate()
		.map(|(i, (field_name, field_type))| ((field_name.into(), i), field_type.into()))
		.unzip();
	TypeBuilder {
		name: name.into(),
		kind: TypeKind::Struct,
		type_parameters: Default::default(),
		field_names,
		field_types,
		variant_names: Default::default(),
		variants: Default::default(),
		functions: Default::default(),
	}
}
