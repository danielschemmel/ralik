use std::sync::Arc;

use crate::error::RuntimeError;
use crate::{Context, Type,Value};

use super::super::arguments::Arguments;

pub(crate) fn add(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_string(context, format!("{}{}", lhs, rhs))?)
}

pub(crate) fn equal(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_bool(context, lhs == rhs)?)
}

pub(crate) fn not_equal(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_bool(context, lhs != rhs)?)
}

pub(crate) fn less(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_bool(context, lhs < rhs)?)
}

pub(crate) fn less_or_equal(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_bool(context, lhs <= rhs)?)
}

pub(crate) fn greater(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_bool(context, lhs > rhs)?)
}

pub(crate) fn greater_or_equal(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_string(0)?;
	let rhs = arguments.as_string(1)?;
	Ok(Value::new_bool(context, lhs >= rhs)?)
}
