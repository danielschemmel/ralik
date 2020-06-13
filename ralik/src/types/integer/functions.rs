use num_traits::{ToPrimitive, Signed};

use crate::{CallError, Context, Value};

use super::super::arguments::Arguments;
use super::super::Overflow;

pub(crate) fn abs(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_integer(context, this.abs())?)
}

/*
pub(crate) fn checked_div(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	let result = lhs.checked_div(rhs);
	Ok(Value::Option(result.map(|value| Box::new(Value::Integer(value)))))
}
*/

pub(crate) fn is_negative(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_bool(context, this.is_negative())?)
}

pub(crate) fn is_positive(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_bool(context, this.is_positive())?)
}

pub(crate) fn pow(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let this = arguments.as_integer(0)?;
	let arg = arguments.as_integer(1)?.to_u32().ok_or_else(|| Overflow::U32)?;
	Ok(Value::new_integer(context, this.pow(arg))?)
}

pub(crate) fn signum(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_integer(context, this.signum())?)
}

pub(crate) fn to_string(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_string(context, this.to_string())?)
}
