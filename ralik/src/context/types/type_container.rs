use std::collections::HashSet;
use std::hash::Hash;
use std::sync::RwLock;

pub(crate) struct TypeContainer {
	pub(super) data: RwLock<HashSet<TypeSetElement>>,
}

use crate::TypeHandle;

impl Default for TypeContainer {
	fn default() -> Self {
		TypeContainer {
			data: RwLock::new(HashSet::new()),
		}
	}
}

impl std::fmt::Debug for TypeContainer {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.data.read().unwrap().fmt(f)
	}
}

#[derive(Clone)]
pub(super) struct TypeSetElement(TypeHandle);

impl Hash for TypeSetElement {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.name().hash(state);
	}
}

impl PartialEq for TypeSetElement {
	fn eq(&self, other: &TypeSetElement) -> bool {
		TypeHandle::is_same(&self.0, &other.0) || self.0.name() == other.0.name()
	}
}
impl Eq for TypeSetElement {}

impl PartialEq<str> for TypeSetElement {
	fn eq(&self, other: &str) -> bool {
		self.0.name() == other
	}
}

impl PartialEq<TypeSetElement> for str {
	fn eq(&self, other: &TypeSetElement) -> bool {
		self == other.0.name()
	}
}

impl std::borrow::Borrow<str> for TypeSetElement {
	fn borrow(&self) -> &str {
		self.0.name()
	}
}

impl From<TypeHandle> for TypeSetElement {
	fn from(value: TypeHandle) -> Self {
		TypeSetElement(value)
	}
}

impl From<TypeSetElement> for TypeHandle {
	fn from(value: TypeSetElement) -> Self {
		value.0
	}
}

impl From<&TypeSetElement> for TypeHandle {
	fn from(value: &TypeSetElement) -> Self {
		value.0.clone()
	}
}

impl<'a> From<&'a TypeSetElement> for &'a TypeHandle {
	fn from(value: &'a TypeSetElement) -> Self {
		&value.0
	}
}

impl std::fmt::Debug for TypeSetElement {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
