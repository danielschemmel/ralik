use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Arc;

use super::Type;

fn make_type() -> Type {
	let functions = HashMap::new();

	Type {
		name: "String".to_string(),
		functions,
	}
}

impl Type {
	pub fn string() -> Arc<Type> {
		lazy_static! {
			static ref TYPE: Arc<Type> = Arc::new(make_type());
		}

		TYPE.clone()
	}
}
