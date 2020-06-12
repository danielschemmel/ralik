use proc_macro2::Span;

use super::{Eval, EvalError};
use crate::{Context, Value};

pub(crate) fn call_member_function_0(name: &str, value: Value, span: &Span) -> Result<Value, EvalError> {
	let r#type = value.get_type();
	let function = r#type.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
		name: format!("{}::{}", r#type.name(), name),
		span: span.clone(),
	})?;
	function(&[value]).map_err(|source| EvalError::MemberCallError {
		name: name.to_string(),
		type_name: r#type.name().to_string(),
		source,
		span: *span,
	})
}

pub(crate) fn call_member_function_1<T: Eval>(
	name: &str,
	value: Value,
	argument: &T,
	span: &Span,
	context: &Context,
) -> Result<Value, EvalError> {
	let argument = argument.eval(context)?;
	let r#type = value.get_type();
	let function = r#type.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
		name: format!("{}::{}", r#type.name(), name),
		span: span.clone(),
	})?;
	function(&[value, argument]).map_err(|source| EvalError::MemberCallError {
		name: name.to_string(),
		type_name: r#type.name().to_string(),
		source,
		span: *span,
	})
}

pub(crate) fn call_member_function_n<T: Eval>(
	name: &str,
	value: Value,
	arguments: &[T],
	span: &Span,
	context: &Context,
) -> Result<Value, EvalError> {
	let r#type = value.get_type();
	let function = r#type.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
		name: format!("{}::{}", r#type.name(), name),
		span: span.clone(),
	})?;
	let arguments = std::iter::once(Ok(value))
		.chain(arguments.iter().map(|argument| argument.eval(context)))
		.collect::<Result<Vec<Value>, EvalError>>()?;
	function(&arguments).map_err(|source| EvalError::MemberCallError {
		name: name.to_string(),
		type_name: r#type.name().to_string(),
		source,
		span: *span,
	})
}
