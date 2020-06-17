use anyhow::anyhow;
use pretty_assertions::assert_eq;

use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn unwrap(_context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?;

	match this.as_variant_id().unwrap() {
		0 => Err(anyhow!("Cannot unwrap option: is `None`").into()),
		1 => {
			let elements = this.as_array().unwrap();
			assert_eq!(elements.len(), 1);
			Ok(elements[0].clone())
		}
		_ => unreachable!(),
	}
}
