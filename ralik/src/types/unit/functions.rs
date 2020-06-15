use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn clone(_context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	arguments.check_type(0, this_type).map(|value| value.clone())
}
