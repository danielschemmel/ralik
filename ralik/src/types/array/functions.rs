use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;

pub(crate) fn len(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_array(0)?;
	Ok(Value::Integer(value.len().into()))
}
