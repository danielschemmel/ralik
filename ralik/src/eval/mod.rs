use super::ast::{AtomicExpression, Expression, Prefix, Suffix};
use super::{Context, Value};

mod error;
pub use error::{CallError, EvalError};

mod member_function;
use member_function::{call_member_function_0, call_member_function_1, call_member_function_n};

pub trait Eval {
	fn eval(&self, context: &Context) -> Result<Value, EvalError>;
}

impl Eval for Expression {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		match self {
			Expression::Atomic(expression) => expression.eval(context),
			Expression::Prefix(expression, prefix) => {
				let value = expression.eval(context)?;
				match prefix {
					Prefix::Not(span) => call_member_function_0(crate::ops::PREFIX_NOT, value, span),
					Prefix::Minus(span) => call_member_function_0(crate::ops::PREFIX_MINUS, value, span),
				}
			}
			Expression::Suffix(expression, suffix) => {
				let value = expression.eval(context)?;
				match suffix {
					Suffix::Unwrap(span) => call_member_function_0(crate::ops::UNWRAP, value, span),
					Suffix::Field(name, span) => value.field(name).cloned().ok_or_else(|| EvalError::InvalidFieldAccess {
						member_name: name.clone(),
						type_name: value.get_type().name().to_string(),
						span: span.clone(),
					}),
					Suffix::TupleIndex(index, span) => {
						let name = index.to_string();
						value
							.field(&name)
							.cloned()
							.ok_or_else(|| EvalError::InvalidFieldAccess {
								member_name: name,
								type_name: value.get_type().name().to_string(),
								span: span.clone(),
							})
					}
					Suffix::ArrayIndex(index, span) => {
						call_member_function_1(crate::ops::ARRAY_INDEX, value, index, span, context)
					}
					Suffix::FunctionCall(name, name_span, arguments, _arguments_span) => {
						call_member_function_n(name, value, &arguments.arguments, name_span, context)
					}
				}
			}
		}
	}
}

impl Eval for AtomicExpression {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		match self {
			AtomicExpression::Parenthesized(expression, _span) => expression.eval(context),
			AtomicExpression::LitBool(value, _span) => Ok(Value::Bool(*value)),
			AtomicExpression::LitChar(value, _span) => Ok(Value::Char(*value)),
			AtomicExpression::LitInt(value, _span) => Ok(Value::Integer(value.clone())),
			AtomicExpression::LitStr(value, _span) => Ok(Value::String(value.clone())),
			AtomicExpression::Dollar(span) => context
				.get_variable("$")
				.cloned()
				.ok_or_else(|| EvalError::UnknownVariable {
					name: "$".to_string(),
					span: span.clone(),
				}),
			AtomicExpression::FunctionCall(name, name_span, arguments, _arguments_span) => {
				let function = context.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
					name: name.clone(),
					span: name_span.clone(),
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				function(&arguments).map_err(|source| EvalError::CallError {
					source,
					span: name_span.clone(),
				})
			}
			AtomicExpression::MacroCall(name, name_span, arguments, _arguments_span) => {
				let macro_function = context.get_macro(name).ok_or_else(|| EvalError::UnknownMacro {
					name: name.clone(),
					span: name_span.clone(),
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				macro_function(&arguments).map_err(|source| EvalError::CallError {
					source,
					span: name_span.clone(),
				})
			}
		}
	}
}

impl<T: Eval> Eval for Box<T> {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		(**self).eval(context)
	}
}
