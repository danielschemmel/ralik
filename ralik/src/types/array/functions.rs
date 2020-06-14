use std::sync::Arc;

use crate::error::{RuntimeError};
use crate::{Context, Type, Value};

use super::super::arguments::Arguments;

pub(crate) fn is_empty(context: &Context, this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?.as_array().unwrap();
	Ok(Value::new_bool(context, this.is_empty())?)
}

pub(crate) fn len(context: &Context, this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?.as_array().unwrap();
	Ok(Value::new_integer(context, this.len())?)
}
