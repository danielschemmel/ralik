use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;

/*
pub(crate) fn checked_div(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0)?;
	let rhs = arguments.as_integer(1)?;
	let result = lhs.checked_div(rhs);
	Ok(Value::Option(result.map(|value| Box::new(Value::Integer(value)))))
}
*/

pub(crate) fn to_string(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_integer(0)?;
	Ok(Value::new_string(context, value.to_string())?)
}
