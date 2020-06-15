use proc_macro2::Span;

use super::{Eval, EvalError};
use crate::{Context, Value};

pub(crate) fn call_member_function_0(
	context: &Context,
	name: &str,
	value: Value,
	span: &Span,
) -> Result<Value, EvalError> {
	let r#type = value.get_type().clone();
	let function = r#type
		.get_function(name)
		.ok_or_else(|| EvalError::UnknownMemberFunction {
			name: name.into(),
			type_name: r#type.name().into(),
			at: span.into(),
		})?;
	function(context, &r#type, &[value]).map_err(|source| EvalError::MemberRuntimeError {
		name: name.into(),
		type_name: r#type.name().into(),
		source,
		at: span.into(),
	})
}

pub(crate) fn call_member_function_1<T: Eval>(
	context: &Context,
	name: &str,
	value: Value,
	argument: &T,
	span: &Span,
) -> Result<Value, EvalError> {
	let argument = argument.eval(context)?;
	let r#type = value.get_type().clone();
	let function = r#type
		.get_function(name)
		.ok_or_else(|| EvalError::UnknownMemberFunction {
			name: name.into(),
			type_name: r#type.name().into(),
			at: span.into(),
		})?;
	function(context, &r#type, &[value, argument]).map_err(|source| EvalError::MemberRuntimeError {
		name: name.into(),
		type_name: r#type.name().into(),
		source,
		at: span.into(),
	})
}

pub(crate) fn call_member_function_n<T: Eval>(
	context: &Context,
	name: &str,
	value: Value,
	arguments: &[T],
	span: &Span,
) -> Result<Value, EvalError> {
	let r#type = value.get_type().clone();
	let function = r#type
		.get_function(name)
		.ok_or_else(|| EvalError::UnknownMemberFunction {
			name: name.into(),
			type_name: r#type.name().into(),
			at: span.into(),
		})?;
	let arguments = std::iter::once(Ok(value))
		.chain(arguments.iter().map(|argument| argument.eval(context)))
		.collect::<Result<Vec<Value>, EvalError>>()?;
	function(context, &r#type, &arguments).map_err(|source| EvalError::MemberRuntimeError {
		name: name.into(),
		type_name: r#type.name().into(),
		source,
		at: span.into(),
	})
}
