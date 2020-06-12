use crate::Value;

use super::super::arguments::Arguments;
use super::super::CallError;

pub(crate) fn not(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	Ok(Value::Bool(!arguments.as_bool(0)?))
}

pub(crate) fn bit_and(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? & arguments.as_bool(1)?))
}

pub(crate) fn bit_or(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? | arguments.as_bool(1)?))
}

pub(crate) fn bit_xor(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? ^ arguments.as_bool(1)?))
}

pub(crate) fn equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? == arguments.as_bool(1)?))
}

pub(crate) fn not_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? != arguments.as_bool(1)?))
}

pub(crate) fn less(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? < arguments.as_bool(1)?))
}

pub(crate) fn less_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? <= arguments.as_bool(1)?))
}

pub(crate) fn greater(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? > arguments.as_bool(1)?))
}

pub(crate) fn greater_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_bool(0)? >= arguments.as_bool(1)?))
}
