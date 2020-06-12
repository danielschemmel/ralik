use crate::Value;

use super::super::arguments::Arguments;
use super::super::CallError;

pub(crate) fn to_string(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_bool(0)?;
	Ok(Value::String(value.to_string()))
}
