use lazy_static::lazy_static;
use num_bigint::Sign;
use num_traits::ToPrimitive;

use std::collections::HashMap;
use std::sync::Arc;

use super::arguments::Arguments;
use super::{CallError, Overflow, MemberFunction, Type, Value};

pub(crate) const NAME: &str = "int";

impl Type {
	pub fn integer() -> Arc<Type> {
		lazy_static! {
			static ref TYPE: Arc<Type> = Arc::new(make_type());
		}

		TYPE.clone()
	}
}

fn make_type() -> Type {
	let mut functions: HashMap<String, MemberFunction> = HashMap::new();

	functions.insert(crate::ops::NOT.to_string(), not);
	functions.insert(crate::ops::NEGATE.to_string(), negate);
	functions.insert(crate::ops::NOT.to_string(), not);
	functions.insert(crate::ops::MUL.to_string(), multiply);
	functions.insert(crate::ops::DIV.to_string(), divide);
	functions.insert(crate::ops::REM.to_string(), remainder);
	functions.insert(crate::ops::ADD.to_string(), add);
	functions.insert(crate::ops::SUB.to_string(), subtract);
	functions.insert(crate::ops::SHL.to_string(), shift_left);
	functions.insert(crate::ops::SHR.to_string(), shift_right);
	functions.insert(crate::ops::BIT_AND.to_string(), bit_and);
	functions.insert(crate::ops::BIT_OR.to_string(), bit_or);
	functions.insert(crate::ops::BIT_XOR.to_string(), bit_xor);
	functions.insert(crate::ops::EQUAL.to_string(), equal);
	functions.insert(crate::ops::NOT_EQUAL.to_string(), not_equal);
	functions.insert(crate::ops::LESS.to_string(), less);
	functions.insert(crate::ops::LESS_OR_EQUAL.to_string(), less_or_equal);
	functions.insert(crate::ops::GREATER.to_string(), greater);
	functions.insert(crate::ops::GREATER_OR_EQUAL.to_string(), greater_or_equal);

	Type {
		name: NAME.to_string(),
		functions,
	}
}

fn not(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::Integer(!value))
}

fn negate(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::Integer(-value))
}

fn multiply(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs * rhs))
}

fn divide(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs / rhs))
}

fn remainder(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs % rhs))
}

fn add(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs + rhs))
}

fn subtract(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs - rhs))
}

fn shift_left(arguments: &[Value]) -> Result<Value, CallError> {
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

fn shift_right(arguments: &[Value]) -> Result<Value, CallError> {
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

fn bit_and(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs & rhs))
}

fn bit_or(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs | rhs))
}

fn bit_xor(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	Ok(Value::Integer(lhs ^ rhs))
}

fn equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? == *arguments.as_integer(1)?))
}

fn not_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? != *arguments.as_integer(1)?))
}

fn less(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? < *arguments.as_integer(1)?))
}

fn less_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? <= *arguments.as_integer(1)?))
}

fn greater(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? > *arguments.as_integer(1)?))
}

fn greater_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_integer(0)? >= *arguments.as_integer(1)?))
}
