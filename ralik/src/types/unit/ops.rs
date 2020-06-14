use crate::error::RuntimeError;
use crate::{Context, Value};

use super::super::arguments::Arguments;

pub(crate) fn equal(context: &Context, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_unit(0)?;
	let rhs = arguments.as_unit(1)?;
	Ok(Value::new_bool(context, lhs == rhs)?)
}

pub(crate) fn not_equal(context: &Context, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_unit(0)?;
	let rhs = arguments.as_unit(1)?;
	Ok(Value::new_bool(context, lhs != rhs)?)
}

pub(crate) fn less(context: &Context, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_unit(0)?;
	let rhs = arguments.as_unit(1)?;
	Ok(Value::new_bool(context, lhs < rhs)?)
}

pub(crate) fn less_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_unit(0)?;
	let rhs = arguments.as_unit(1)?;
	Ok(Value::new_bool(context, lhs <= rhs)?)
}

pub(crate) fn greater(context: &Context, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_unit(0)?;
	let rhs = arguments.as_unit(1)?;
	Ok(Value::new_bool(context, lhs > rhs)?)
}

pub(crate) fn greater_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_unit(0)?;
	let rhs = arguments.as_unit(1)?;
	Ok(Value::new_bool(context, lhs >= rhs)?)
}
