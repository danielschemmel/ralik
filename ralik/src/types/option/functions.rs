use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;

pub(crate) fn is_none(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_option(0)?;
	Ok(Value::Bool(value.is_none()))
}

pub(crate) fn is_some(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_option(0)?;
	Ok(Value::Bool(value.is_some()))
}
