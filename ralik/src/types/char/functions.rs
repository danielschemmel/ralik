use num_traits::ToPrimitive;

use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;
use super::super::Overflow;

pub(crate) fn eq_ignore_ascii_case(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_char(0)?;
	let arg = arguments.as_char(1)?;
	Ok(Value::new_bool(context, this.eq_ignore_ascii_case(&arg))?)
}

pub(crate) fn is_alphabetic(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_alphabetic())?)
}

pub(crate) fn is_alphanumeric(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_alphanumeric())?)
}

pub(crate) fn is_ascii(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii())?)
}

pub(crate) fn is_ascii_alphabetic(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_alphabetic())?)
}

pub(crate) fn is_ascii_alphanumeric(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_alphanumeric())?)
}

pub(crate) fn is_ascii_control(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_control())?)
}

pub(crate) fn is_ascii_digit(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_digit())?)
}

pub(crate) fn is_ascii_graphic(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_graphic())?)
}

pub(crate) fn is_ascii_hexdigit(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_hexdigit())?)
}

pub(crate) fn is_ascii_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_lowercase())?)
}

pub(crate) fn is_ascii_punctuation(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_punctuation())?)
}

pub(crate) fn is_ascii_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_uppercase())?)
}

pub(crate) fn is_ascii_whitespace(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_ascii_whitespace())?)
}

pub(crate) fn is_control(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_control())?)
}

pub(crate) fn is_digit(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_char(0)?;
	let arg = arguments.as_integer(0)?.to_u32().ok_or_else(|| Overflow::U32)?;
	Ok(Value::new_bool(context, this.is_digit(arg))?)
}

pub(crate) fn is_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_lowercase())?)
}

pub(crate) fn is_numeric(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_numeric())?)
}

pub(crate) fn is_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_uppercase())?)
}

pub(crate) fn is_whitespace(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_bool(context, this.is_whitespace())?)
}

pub(crate) fn len_utf16(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_integer(context, this.len_utf16())?)
}

pub(crate) fn len_utf8(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_integer(context, this.len_utf8())?)
}

pub(crate) fn to_ascii_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_char(context, this.to_ascii_lowercase())?)
}

pub(crate) fn to_ascii_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_char(context, this.to_ascii_uppercase())?)
}

pub(crate) fn to_string(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_char(0)?;
	Ok(Value::new_string(context, this.to_string())?)
}
