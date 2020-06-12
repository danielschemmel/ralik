use crate::Value;

use super::super::arguments::Arguments;
use super::super::CallError;

pub(crate) fn equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_char(0)? == arguments.as_char(1)?))
}

pub(crate) fn not_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_char(0)? != arguments.as_char(1)?))
}

pub(crate) fn less(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_char(0)? < arguments.as_char(1)?))
}

pub(crate) fn less_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_char(0)? <= arguments.as_char(1)?))
}

pub(crate) fn greater(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_char(0)? > arguments.as_char(1)?))
}

pub(crate) fn greater_or_equal(arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(2)?;
	Ok(Value::Bool(arguments.as_char(0)? >= arguments.as_char(1)?))
}
