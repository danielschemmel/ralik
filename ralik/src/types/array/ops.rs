use num::ToPrimitive;

use crate::error::{Overflow, RuntimeError};
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn index(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.check_type(0, this_type)?.as_array().unwrap();
	let arg = arguments
		.as_integer(1, context)?
		.to_usize()
		.ok_or_else(|| Overflow::USize)?;
	Ok(this.get(arg).cloned().ok_or_else(|| RuntimeError::OutOfBounds {
		index: arg,
		len: this.len(),
	})?)
}
