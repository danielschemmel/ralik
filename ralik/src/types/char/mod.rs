use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Arc;

use super::{MemberFunction, Type};

mod functions;
mod ops;

pub(crate) const NAME: &str = "char";

impl Type {
	pub fn char() -> Arc<Type> {
		lazy_static! {
			static ref TYPE: Arc<Type> = Arc::new(make_type());
		}

		TYPE.clone()
	}
}

fn make_type() -> Type {
	let mut functions: HashMap<String, MemberFunction> = HashMap::new();
	
	functions.insert(crate::ops::EQUAL.to_string(), ops::equal);
	functions.insert(crate::ops::NOT_EQUAL.to_string(), ops::not_equal);
	functions.insert(crate::ops::LESS.to_string(), ops::less);
	functions.insert(crate::ops::LESS_OR_EQUAL.to_string(), ops::less_or_equal);
	functions.insert(crate::ops::GREATER.to_string(), ops::greater);
	functions.insert(crate::ops::GREATER_OR_EQUAL.to_string(), ops::greater_or_equal);

	functions.insert("to_string".to_string(), functions::to_string);

	Type {
		name: NAME.to_string(),
		functions,
	}
}
