use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn equal(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let _lhs = arguments.check_type(0, this_type)?;
	let _rhs = arguments.check_type(1, this_type)?;
	Ok(Value::new_bool(context, () == ())?)
}

pub(crate) fn not_equal(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let _lhs = arguments.check_type(0, this_type)?;
	let _rhs = arguments.check_type(1, this_type)?;
	Ok(Value::new_bool(context, () != ())?)
}

pub(crate) fn less(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let _lhs = arguments.check_type(0, this_type)?;
	let _rhs = arguments.check_type(1, this_type)?;
	Ok(Value::new_bool(context, () < ())?)
}

pub(crate) fn less_or_equal(
	context: &Context,
	this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let _lhs = arguments.check_type(0, this_type)?;
	let _rhs = arguments.check_type(1, this_type)?;
	Ok(Value::new_bool(context, () <= ())?)
}

pub(crate) fn greater(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let _lhs = arguments.check_type(0, this_type)?;
	let _rhs = arguments.check_type(1, this_type)?;
	Ok(Value::new_bool(context, () > ())?)
}

pub(crate) fn greater_or_equal(
	context: &Context,
	this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let _lhs = arguments.check_type(0, this_type)?;
	let _rhs = arguments.check_type(1, this_type)?;
	Ok(Value::new_bool(context, () >= ())?)
}
