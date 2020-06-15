use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

pub(crate) type TupleType = BasicType<TupleImpl>;

pub(crate) struct TupleImpl {
	name: String,
	element_types: Vec<TypeHandle>,
}

pub fn name(element_types: &[&str]) -> String {
	assert!(
		element_types.len() > 0,
		"Empty tuples do not exist (see also \"Unit Type\")"
	);

	let mut name = "(".to_owned();
	for (i, &element_type_name) in element_types.iter().enumerate() {
		if i > 0 {
			name.push_str(", ");
		}
		name.push_str(element_type_name);
	}
	name.push_str(")");

	name
}

impl TupleType {
	pub fn new(name: impl Into<String>, element_types: impl Into<Vec<TypeHandle>>) -> Self {
		Self::from_base(TupleImpl {
			name: name.into(),
			element_types: element_types.into(),
		})
	}
}

impl BasicTypeBase for TupleImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Tuple
	}

	fn type_parameters(&self) -> &[TypeHandle] {
		&self.element_types
	}
}
