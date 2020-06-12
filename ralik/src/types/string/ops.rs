use crate::Value;

use super::super::arguments::Arguments;
use super::super::CallError;

pub(crate) fn add(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::String(format!("{}{}", lhs, rhs)))
}

pub(crate) fn equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_string(0)? == *arguments.as_string(1)?))
}

pub(crate) fn not_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_string(0)? != *arguments.as_string(1)?))
}

pub(crate) fn less(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_string(0)? < *arguments.as_string(1)?))
}

pub(crate) fn less_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_string(0)? <= *arguments.as_string(1)?))
}

pub(crate) fn greater(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_string(0)? > *arguments.as_string(1)?))
}

pub(crate) fn greater_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_string(0)? >= *arguments.as_string(1)?))
}
