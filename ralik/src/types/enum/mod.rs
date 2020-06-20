use super::{TypeBuilder, TypeKind, VariantBuilder};

//mod functions;
//mod ops;

pub fn new_enum_type(name: impl Into<String>, variants: impl Into<Vec<VariantBuilder>>) -> TypeBuilder {
	TypeBuilder {
		name: name.into(),
		kind: TypeKind::Enum,
		type_parameters: Default::default(),
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: variants.into(),
		functions: Default::default(),
	}
}
