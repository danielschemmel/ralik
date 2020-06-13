use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;

pub(crate) fn to_string(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_char(0)?;
	Ok(Value::new_string(context, value.to_string())?)
}
