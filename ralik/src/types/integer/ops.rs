use num_bigint::Sign;
use num_traits::ToPrimitive;

use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;
use super::super::{Overflow};

pub(crate) fn not(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::new_integer(context,!value)?)
}

pub(crate) fn negate(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::new_integer(context,-value)?)
}

pub(crate) fn multiply(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs * rhs)?)
}

pub(crate) fn divide(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs / rhs)?)
}

pub(crate) fn remainder(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs % rhs)?)
}

pub(crate) fn add(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs + rhs)?)
}

pub(crate) fn subtract(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs - rhs)?)
}

pub(crate) fn shift_left(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	if rhs.sign() == Sign::Minus {
		Err(Overflow::NegativeShift.into())
	} else {
		let rhs = rhs.to_usize().ok_or_else(|| Overflow::LargeShift)?;
		Ok(Value::new_integer(context,lhs << rhs)?)
	}
}

pub(crate) fn shift_right(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	if rhs.sign() == Sign::Minus {
		Err(Overflow::NegativeShift.into())
	} else {
		let rhs = rhs.to_usize().ok_or_else(|| Overflow::LargeShift)?;
		Ok(Value::new_integer(context,lhs >> rhs)?)
	}
}

pub(crate) fn bit_and(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs & rhs)?)
}

pub(crate) fn bit_or(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs | rhs)?)
}

pub(crate) fn bit_xor(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_integer(context,lhs ^ rhs)?)
}

pub(crate) fn equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_bool(context, lhs == rhs)?)
}

pub(crate) fn not_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_bool(context, lhs != rhs)?)
}

pub(crate) fn less(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_bool(context, lhs < rhs)?)
}

pub(crate) fn less_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_bool(context, lhs <= rhs)?)
}

pub(crate) fn greater(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_bool(context, lhs > rhs)?)
}

pub(crate) fn greater_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::new_bool(context, lhs >= rhs)?)
}
