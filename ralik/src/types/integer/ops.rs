use num_bigint::Sign;
use num_traits::ToPrimitive;

use crate::Value;

use super::super::arguments::Arguments;
use super::super::{CallError, Overflow};

pub(crate) fn not(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::Integer(!value))
}

pub(crate) fn negate(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::Integer(-value))
}

pub(crate) fn multiply(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs * rhs))
}

pub(crate) fn divide(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs / rhs))
}

pub(crate) fn remainder(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs % rhs))
}

pub(crate) fn add(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs + rhs))
}

pub(crate) fn subtract(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs - rhs))
}

pub(crate) fn shift_left(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	if rhs.sign() == Sign::Minus {
		Err(Overflow::NegativeShift.into())
	} else {
		let rhs = rhs.to_usize().ok_or_else(|| Overflow::LargeShift)?;
		Ok(Value::Integer(lhs << rhs))
	}
}

pub(crate) fn shift_right(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	if rhs.sign() == Sign::Minus {
		Err(Overflow::NegativeShift.into())
	} else {
		let rhs = rhs.to_usize().ok_or_else(|| Overflow::LargeShift)?;
		Ok(Value::Integer(lhs >> rhs))
	}
}

pub(crate) fn bit_and(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs & rhs))
}

pub(crate) fn bit_or(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs | rhs))
}

pub(crate) fn bit_xor(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs ^ rhs))
}

pub(crate) fn equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? == *arguments.as_integer(1)?))
}

pub(crate) fn not_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? != *arguments.as_integer(1)?))
}

pub(crate) fn less(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? < *arguments.as_integer(1)?))
}

pub(crate) fn less_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? <= *arguments.as_integer(1)?))
}

pub(crate) fn greater(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? > *arguments.as_integer(1)?))
}

pub(crate) fn greater_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? >= *arguments.as_integer(1)?))
}
