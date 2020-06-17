use num::bigint::Sign;
use num::ToPrimitive;

use crate::error::{Overflow, RuntimeError};
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn not(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0, context)?;
	Ok(Value::new_integer(context, !value)?)
}

pub(crate) fn negate(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0, context)?;
	Ok(Value::new_integer(context, -value)?)
}

pub(crate) fn multiply(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs * rhs)?)
}

pub(crate) fn divide(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs / rhs)?)
}

pub(crate) fn remainder(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs % rhs)?)
}

pub(crate) fn add(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs + rhs)?)
}

pub(crate) fn subtract(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs - rhs)?)
}

pub(crate) fn shift_left(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	if rhs.sign() == Sign::Minus {
		Err(Overflow::NegativeShift.into())
	} else {
		let rhs = rhs.to_usize().ok_or_else(|| Overflow::LargeShift)?;
		Ok(Value::new_integer(context, lhs << rhs)?)
	}
}

pub(crate) fn shift_right(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	if rhs.sign() == Sign::Minus {
		Err(Overflow::NegativeShift.into())
	} else {
		let rhs = rhs.to_usize().ok_or_else(|| Overflow::LargeShift)?;
		Ok(Value::new_integer(context, lhs >> rhs)?)
	}
}

pub(crate) fn bit_and(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs & rhs)?)
}

pub(crate) fn bit_or(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs | rhs)?)
}

pub(crate) fn bit_xor(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_integer(context, lhs ^ rhs)?)
}

pub(crate) fn equal(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_bool(context, lhs == rhs)?)
}

pub(crate) fn not_equal(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_bool(context, lhs != rhs)?)
}

pub(crate) fn less(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_bool(context, lhs < rhs)?)
}

pub(crate) fn less_or_equal(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_bool(context, lhs <= rhs)?)
}

pub(crate) fn greater(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_bool(context, lhs > rhs)?)
}

pub(crate) fn greater_or_equal(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	Ok(Value::new_bool(context, lhs >= rhs)?)
}
