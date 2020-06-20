use super::{TypeBuilder, TypeKind};

//mod functions;
//mod ops;

pub fn new_unit_struct_type(name: impl Into<String>) -> TypeBuilder {
	TypeBuilder {
		name: name.into(),
		kind: TypeKind::UnitStruct,
		type_parameters: Default::default(),
		field_names: Default::default(),
		field_types: Default::default(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: Default::default(),
	}
}
