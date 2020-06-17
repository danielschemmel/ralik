use std::collections::HashMap;

use super::{MemberFunction, Type, TypeHandle, TypeKind, Variant};

pub trait BasicTypeBase {
	fn name(&self) -> &str;
	fn kind(&self) -> TypeKind;

	fn type_parameters(&self) -> &[TypeHandle] {
		assert!(self.kind() != TypeKind::Tuple);
		assert!(self.kind() != TypeKind::Array);
		&[]
	}

	fn fields(&self) -> (Option<&HashMap<Box<str>, usize>>, &[TypeHandle]) {
		assert!(self.kind() != TypeKind::Struct && self.kind() != TypeKind::Tuple);
		(None, &[])
	}

	fn variants(&self) -> Option<(&HashMap<Box<str>, usize>, &[Variant])> {
		assert!(self.kind() != TypeKind::Enum);
		None
	}
}

pub struct BasicType<T: BasicTypeBase> {
	base: T,
	functions: HashMap<Box<str>, MemberFunction>,
}

impl<T: BasicTypeBase> BasicType<T> {
	pub fn from_base(base: T) -> Self {
		Self {
			base,
			functions: HashMap::new(),
		}
	}

	pub fn from_base_with_functions(base: T, functions: Vec<(impl Into<Box<str>>, MemberFunction)>) -> Self {
		Self {
			base,
			functions: functions
				.into_iter()
				.map(|(key, function)| (key.into(), function))
				.collect(),
		}
	}

	pub fn insert_function(&mut self, key: impl Into<Box<str>>, value: MemberFunction) -> Option<MemberFunction> {
		self.functions.insert(key.into(), value)
	}
}

impl<T: BasicTypeBase> Type for BasicType<T> {
	fn name(&self) -> &str {
		self.base.name()
	}

	fn kind(&self) -> TypeKind {
		self.base.kind()
	}

	fn type_parameters(&self) -> &[TypeHandle] {
		self.base.type_parameters()
	}

	fn fields(&self) -> (Option<&HashMap<Box<str>, usize>>, &[TypeHandle]) {
		self.base.fields()
	}

	fn variants(&self) -> Option<(&HashMap<Box<str>, usize>, &[Variant])> {
		self.base.variants()
	}

	fn get_function(&self, key: &str) -> Option<&MemberFunction> {
		self.functions.get(key)
	}

	fn insert_function(&mut self, key: Box<str>, value: MemberFunction) -> Option<MemberFunction> {
		self.functions.insert(key, value)
	}

	fn remove_function(&mut self, key: &str) -> Option<(Box<str>, MemberFunction)> {
		self.functions.remove_entry(key)
	}
}

impl<T: BasicTypeBase> std::fmt::Debug for BasicType<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Type")
			.field("name", &self.name())
			.field("kind", &self.kind())
			.field("type_parameters", &self.type_parameters())
			.field("fields", &self.fields())
			.field("variants", &self.variants())
			.field("functions", &MapKeySequence(&self.functions))
			.finish()
	}
}
struct MapKeySequence<'a, T>(&'a HashMap<Box<str>, T>);

impl<'a, T> std::fmt::Debug for MapKeySequence<'a, T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.keys()).finish()
	}
}
