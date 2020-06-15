use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

pub(crate) fn clone(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	assert!(this_type.kind() == crate::types::TypeKind::Array);
	let element_type = &this_type.type_parameters()[0];

	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?.as_array().unwrap();

	// FIXME: how to runtime call functions (that may not exist)?
	let function = element_type.get_function("clone").unwrap();
	let values = this
		.iter()
		// FIXME: the member function signature seems like it does a lot of copying
		.map(|element| function(context, element_type, &[element.clone()]))
		.collect::<Result<Vec<Value>, RuntimeError>>()?;
	Ok(Value::new_array(context, element_type, values)?)
}

pub(crate) fn is_empty(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?.as_array().unwrap();
	Ok(Value::new_bool(context, this.is_empty())?)
}

pub(crate) fn len(context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.check_type(0, this_type)?.as_array().unwrap();
	Ok(Value::new_integer(context, this.len())?)
}
