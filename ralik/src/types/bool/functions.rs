use std::sync::Arc;

use crate::error::RuntimeError;
use crate::{Context, Value, Type};

use super::super::arguments::Arguments;

pub(crate) fn to_string(context: &Context, _this_type: &Arc<dyn Type>, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_bool(0)?;
	Ok(Value::new_string(context, value.to_string())?)
}
