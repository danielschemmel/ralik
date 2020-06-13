use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;

pub(crate) fn not(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_bool(0)?;
	Ok(Value::new_bool(context, !value)?)
}

pub(crate) fn bit_and(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs & rhs)?)
}

pub(crate) fn bit_or(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs | rhs)?)
}

pub(crate) fn bit_xor(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs ^ rhs)?)
}

pub(crate) fn equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs == rhs)?)
}

pub(crate) fn not_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs != rhs)?)
}

pub(crate) fn less(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs < rhs)?)
}

pub(crate) fn less_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs <= rhs)?)
}

pub(crate) fn greater(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs > rhs)?)
}

pub(crate) fn greater_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_bool(0)?;
	let rhs = arguments.as_bool(1)?;
	Ok(Value::new_bool(context, lhs >= rhs)?)
}
