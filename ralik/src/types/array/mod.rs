use super::{BasicType, BasicTypeBase, TypeHandle, TypeKind};

mod functions;
mod ops;

pub(crate) type ArrayType = BasicType<ArrayImpl>;

pub(crate) struct ArrayImpl {
	name: String,
	element_type: [TypeHandle; 1],
}

pub fn name(element_type: &str) -> String {
	format!("[{}]", element_type)
}

impl ArrayType {
	pub fn new(name: impl Into<String>, element_type: TypeHandle) -> Self {
		Self::from_base_with_functions(
			ArrayImpl {
				name: name.into(),
				element_type: [element_type],
			},
			vec![
				(crate::ops::INDEX, ops::index),
				("clone", functions::clone), // FIXME: only insert if it makes sense
				("is_empty", functions::is_empty),
				("len", functions::len),
			],
		)
	}
}

impl BasicTypeBase for ArrayImpl {
	fn name(&self) -> &str {
		&self.name
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Array
	}

	fn type_parameters(&self) -> &[TypeHandle] {
		&self.element_type
	}
}
