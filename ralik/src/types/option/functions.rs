use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn is_none(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?;

	match this.as_variant_id().unwrap() {
		0 => Ok(Value::new_bool(context, true)?),
		1 => Ok(Value::new_bool(context, false)?),
		_ => unreachable!(),
	}
}

pub(crate) fn is_some(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?;

	match this.as_variant_id().unwrap() {
		0 => Ok(Value::new_bool(context, false)?),
		1 => Ok(Value::new_bool(context, true)?),
		_ => unreachable!(),
	}
}
