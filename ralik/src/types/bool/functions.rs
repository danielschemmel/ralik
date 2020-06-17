use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn clone(_context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	arguments.check_type(0, this_type).map(|value| value.clone())
}

pub(crate) fn to_string(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_bool(0, context)?;
	Ok(Value::new_string(context, value.to_string())?)
}
