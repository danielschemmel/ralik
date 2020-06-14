use core::ops::Deref;
use std::sync::Arc;

use super::Type;

#[derive(Clone, Debug)]
pub struct TypeHandle {
	handle: Arc<dyn Type>,
}

impl TypeHandle {
	pub fn new(r#type: impl Type + 'static) -> Self {
		Self {
			handle: Arc::new(r#type),
		}
	}

	pub fn is_same(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.handle, &other.handle)
	}
}

impl Deref for TypeHandle {
	type Target = dyn Type;

	fn deref(&self) -> &Self::Target {
		Arc::deref(&self.handle)
	}
}
