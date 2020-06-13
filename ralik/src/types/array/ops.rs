use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;

pub(crate) fn equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	/*arguments.check_len(2)?;
	let lhs = arguments.as_array(0)?;
	let rhs = arguments.as_array(1)?;
	if lhs.len() != rhs.len() {
		return Ok(Value::Bool(false));
	}

	let result = lhs.iter().zip(rhs.iter()).filter_map(|(lhs, rhs)| {
		let function = lhs.get_type().get_function(crate::ops::EQUAL).ok_or_else(|| CallError::)?;
		function(&[lhs, rhs])
	});

	Ok(Value::Bool(true))*/
	unimplemented!()
}

pub(crate) fn not_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_array(0)? != *arguments.as_array(1)?))
}

pub(crate) fn less(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_array(0)? < *arguments.as_array(1)?))
}

pub(crate) fn less_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_array(0)? <= *arguments.as_array(1)?))
}

pub(crate) fn greater(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_array(0)? > *arguments.as_array(1)?))
}

pub(crate) fn greater_or_equal(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(*arguments.as_array(0)? >= *arguments.as_array(1)?))
}
