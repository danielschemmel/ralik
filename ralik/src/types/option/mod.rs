use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Arc;

use super::{MemberFunction, Type};

mod functions;
mod ops;

pub(crate) const NAME: &str = "Option";

impl Type {
	pub fn option() -> Arc<Type> {
		lazy_static! {
			static ref TYPE: Arc<Type> = Arc::new(make_type());
		}

		TYPE.clone()
	}
}

fn make_type() -> Type {
	let mut functions: HashMap<String, MemberFunction> = HashMap::new();

	functions.insert(crate::ops::UNWRAP.to_string(), ops::unwrap);

	functions.insert("is_none".to_string(), functions::is_none);
	functions.insert("is_some".to_string(), functions::is_some);

	Type {
		name: NAME.to_string(),
		functions,
	}
}
