use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Arc;

use super::{MemberFunction, Type};

mod functions;
mod ops;

pub(crate) const NAME: &str = "Integer";

impl Type {
	pub fn integer() -> Arc<Type> {
		lazy_static! {
			static ref TYPE: Arc<Type> = Arc::new(make_type());
		}

		TYPE.clone()
	}
}

fn make_type() -> Type {
	let mut functions: HashMap<String, MemberFunction> = HashMap::new();

	functions.insert(crate::ops::NOT.to_string(), ops::not);
	functions.insert(crate::ops::NEGATE.to_string(), ops::negate);
	functions.insert(crate::ops::NOT.to_string(), ops::not);
	functions.insert(crate::ops::MUL.to_string(), ops::multiply);
	functions.insert(crate::ops::DIV.to_string(), ops::divide);
	functions.insert(crate::ops::REM.to_string(), ops::remainder);
	functions.insert(crate::ops::ADD.to_string(), ops::add);
	functions.insert(crate::ops::SUB.to_string(), ops::subtract);
	functions.insert(crate::ops::SHL.to_string(), ops::shift_left);
	functions.insert(crate::ops::SHR.to_string(), ops::shift_right);
	functions.insert(crate::ops::BIT_AND.to_string(), ops::bit_and);
	functions.insert(crate::ops::BIT_OR.to_string(), ops::bit_or);
	functions.insert(crate::ops::BIT_XOR.to_string(), ops::bit_xor);
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
