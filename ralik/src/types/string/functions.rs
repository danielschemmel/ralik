use anyhow::anyhow;
use num::ToPrimitive;

use crate::error::{Overflow, RuntimeError};
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn as_bytes(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	let array = this
		.as_bytes()
		.iter()
		.map(|byte| Value::new_integer(context, *byte))
		.collect::<Result<Vec<_>, _>>()?;

	Ok(Value::new_array(
		context,
		&context.get_integer_type().map_err(|err| anyhow!(err))?,
		array,
	)?)
}

pub(crate) fn clone(_context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	arguments.check_type(0, this_type).map(|value| value.clone())
}

pub(crate) fn eq_ignore_ascii_case(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_string(0, context)?;
	let arg = arguments.as_string(1, context)?;
	Ok(Value::new_bool(context, this.eq_ignore_ascii_case(&arg))?)
}

pub(crate) fn is_ascii(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_bool(context, this.is_ascii())?)
}

pub(crate) fn is_char_boundary(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_string(0, context)?;
	let arg = arguments
		.as_integer(1, context)?
		.to_usize()
		.ok_or_else(|| Overflow::USize)?;
	Ok(Value::new_bool(context, this.is_char_boundary(arg))?)
}

pub(crate) fn is_empty(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_bool(context, this.is_empty())?)
}

pub(crate) fn len(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_string(0, context)?;
	Ok(Value::new_integer(context, value.len())?)
}

pub(crate) fn repeat(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_string(0, context)?;
	let arg = arguments
		.as_integer(1, context)?
		.to_usize()
		.ok_or_else(|| Overflow::USize)?;
	Ok(Value::new_string(context, this.repeat(arg))?)
}

pub(crate) fn to_ascii_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_ascii_lowercase())?)
}

pub(crate) fn to_ascii_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_ascii_uppercase())?)
}

pub(crate) fn to_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_lowercase())?)
}

pub(crate) fn to_string(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, value.to_string())?)
}

pub(crate) fn to_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_uppercase())?)
}

pub(crate) fn trim(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.trim())?)
}

pub(crate) fn trim_end(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.trim_end())?)
}

pub(crate) fn trim_start(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.trim_start())?)
}
