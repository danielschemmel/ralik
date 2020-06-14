use num_traits::{Signed, ToPrimitive};

use std::sync::Arc;

use crate::error::{RuntimeError, Overflow};
use crate::{Context, Value, Type};

use super::super::arguments::Arguments;

pub(crate) fn abs(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_integer(context, this.abs())?)
}

/*
pub(crate) fn checked_div(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	let result = lhs.checked_div(rhs);
	Ok(Value::Option(result.map(|value| Box::new(Value::Integer(value)))))
}
*/

pub(crate) fn is_negative(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_bool(context, this.is_negative())?)
}

pub(crate) fn is_positive(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_bool(context, this.is_positive())?)
}

pub(crate) fn pow(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_integer(0)?;
	let arg = arguments.as_integer(1)?.to_u32().ok_or_else(|| Overflow::U32)?;
	Ok(Value::new_integer(context, this.pow(arg))?)
}

pub(crate) fn signum(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_integer(context, this.signum())?)
}

pub(crate) fn to_string(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0)?;
	Ok(Value::new_string(context, this.to_string())?)
}
