use std::collections::hash_map::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::types::{MemberFunction, TypeKind, Variant};

use super::super::{Context, TypeId};

#[derive(Eq, PartialEq)]
pub struct TypeHandle {
	context: Context,
	type_id: TypeId,
}

impl TypeHandle {
	pub fn name(&self) -> Arc<str> {
		self.context.0.types.read().unwrap()[self.type_id.0].0.name.clone()
	}

	pub fn kind(&self) -> TypeKind {
		self.context.0.types.read().unwrap()[self.type_id.0].0.kind
	}

	pub(crate) fn type_parameters(&self) -> Arc<[TypeId]> {
		self.context.0.types.read().unwrap()[self.type_id.0]
			.0
			.type_parameters
			.clone()
	}

	pub fn get_function(&self, name: impl AsRef<str>) -> Option<MemberFunction> {
		self.context.0.types.read().unwrap()[self.type_id.0]
			.0
			.functions
			.get(name.as_ref())
			.cloned()
	}

	pub(crate) fn fields(&self) -> (Arc<HashMap<Box<str>, usize>>, Arc<[TypeId]>) {
		let types = self.context.0.types.read().unwrap();
		(
			types[self.type_id.0].0.field_names.clone(),
			types[self.type_id.0].0.field_types.clone(),
		)
	}

	pub(crate) fn variants(&self) -> (Arc<HashMap<Box<str>, usize>>, Arc<[Variant]>) {
		let types = self.context.0.types.read().unwrap();
		(
			types[self.type_id.0].0.variant_names.clone(),
			types[self.type_id.0].0.variants.clone(),
		)
	}

	pub fn is_same(&self, other: &Self) -> bool {
		assert!(Arc::ptr_eq(&self.context.0, &other.context.0));
		self.type_id.0 == other.type_id.0
	}

	pub(crate) fn refers_to(&self, other: TypeId) -> bool {
		self.type_id.0 == other.0
	}
}

impl std::fmt::Debug for TypeHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<Type {}>", self.type_id.0)
	}
}

impl TypeHandle {
	pub(crate) fn from_type_id(context: impl Into<Context>, type_id: TypeId) -> Self {
		let context = context.into();
		{
			let types = context.0.types.read().unwrap();
			let previous = types[type_id.0].1.fetch_add(1, Ordering::SeqCst);
			if previous == isize::MAX || previous <= 0 {
				types[type_id.0].1.fetch_sub(1, Ordering::Relaxed);
				panic!("Reference count overflow!");
			}
		}

		Self { context, type_id }
	}
}

impl Clone for TypeHandle {
	fn clone(&self) -> Self {
		TypeHandle::from_type_id(self.context.clone(), self.type_id)
	}
}

impl Drop for TypeHandle {
	fn drop(&mut self) {
		let previous = self.context.0.types.read().unwrap()[self.type_id.0]
			.1
			.fetch_sub(1, Ordering::SeqCst);
		debug_assert!(previous != 0);
	}
}
