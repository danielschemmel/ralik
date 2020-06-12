use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Arc;

use super::arguments::Arguments;
use super::{CallError, MemberFunction, Type, Value};

pub(crate) const NAME: &str = "bool";

impl Type {
	pub fn bool() -> Arc<Type> {
		lazy_static! {
			static ref TYPE: Arc<Type> = Arc::new(make_type());
		}

		TYPE.clone()
	}
}

fn make_type() -> Type {
	let mut functions: HashMap<String, MemberFunction> = HashMap::new();

	functions.insert(crate::ops::NOT.to_string(), not);
	functions.insert(crate::ops::BIT_AND.to_string(), bit_and);
	functions.insert(crate::ops::BIT_OR.to_string(), bit_or);
	functions.insert(crate::ops::BIT_XOR.to_string(), bit_xor);
	functions.insert(crate::ops::EQUAL.to_string(), equal);
	functions.insert(crate::ops::NOT_EQUAL.to_string(), not_equal);
	functions.insert(crate::ops::LESS.to_string(), less);
	functions.insert(crate::ops::LESS_OR_EQUAL.to_string(), less_or_equal);
	functions.insert(crate::ops::GREATER.to_string(), greater);
	functions.insert(crate::ops::GREATER_OR_EQUAL.to_string(), greater_or_equal);

	Type {
		name: NAME.to_string(),
		functions,
	}
}

fn not(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	Ok(Value::Bool(!arguments.as_bool(0)?))
}

fn bit_and(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? & arguments.as_bool(1)?))
}

fn bit_or(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? | arguments.as_bool(1)?))
}

fn bit_xor(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? ^ arguments.as_bool(1)?))
}

fn equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? == arguments.as_bool(1)?))
}

fn not_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? != arguments.as_bool(1)?))
}

fn less(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? < arguments.as_bool(1)?))
}

fn less_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? <= arguments.as_bool(1)?))
}

fn greater(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? > arguments.as_bool(1)?))
}

fn greater_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? >= arguments.as_bool(1)?))
}
