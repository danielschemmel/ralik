use super::{TypeBuilder, TypeKind};

//mod functions;
//mod ops;

pub fn new_tuple_struct_type(name: impl Into<String>, field_types: impl Into<Vec<String>>) -> TypeBuilder {
	TypeBuilder {
		name: name.into(),
		kind: TypeKind::TupleStruct,
		type_parameters: Default::default(),
		field_names: Default::default(),
		field_types: field_types.into(),
		variant_names: Default::default(),
		variants: Default::default(),
		functions: Default::default(),
	}
}
