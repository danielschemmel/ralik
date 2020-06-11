use super::ast::{AtomicExpression, Expression, Prefix, Suffix};
use super::{Context, Value};

mod error;
pub use error::{CallError, EvalError};

pub trait Eval {
	fn eval(&self, context: &Context) -> Result<Value, EvalError>;
}

impl Eval for Expression {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		match self {
			Expression::Atomic(expression) => expression.eval(context),
			Expression::Prefix(expression, prefix) => {
				let value = expression.eval(context)?;
				let r#type = value.get_type();
				match prefix {
					Prefix::Not(span) => r#type
						.call(crate::ops::PREFIX_NOT, &[value])
						.map_err(|source| EvalError::CallError {
							source,
							span: span.clone(),
						}),
					Prefix::Minus(span) => {
						r#type
							.call(crate::ops::PREFIX_MINUS, &[value])
							.map_err(|source| EvalError::CallError {
								source,
								span: span.clone(),
							})
					}
				}
			}
			Expression::Suffix(expression, suffix) => {
				let value = expression.eval(context)?;
				match suffix {
					Suffix::Unwrap(span) => {
						let r#type = value.get_type();
						r#type
							.call(crate::ops::UNWRAP, &[value])
							.map_err(|source| EvalError::CallError {
								source,
								span: span.clone(),
							})
					}
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
						let r#type = value.get_type();
						r#type
							.call(crate::ops::ARRAY_INDEX, &[value, index.eval(context)?])
							.map_err(|source| EvalError::CallError {
								source,
								span: span.clone(),
							})
					}
					Suffix::FunctionCall(name, name_span, arguments, _arguments_span) => {
						let r#type = value.get_type();
						let arguments = std::iter::once(Ok(value))
							.chain(arguments.arguments.iter().map(|argument| argument.eval(context)))
							.collect::<Result<Vec<Value>, EvalError>>()?;
						r#type.call(name, &arguments).map_err(|source| EvalError::CallError {
							source,
							span: name_span.clone(),
						})
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
			AtomicExpression::FunctionCall(name, arguments, span) => {
				let function = context.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
					name: name.clone(),
					span: span.clone(),
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				function(&arguments).map_err(|source| EvalError::CallError {
					source,
					span: span.clone(),
				})
			}
			AtomicExpression::MacroCall(name, arguments, span) => {
				let macro_function = context.get_macro(name).ok_or_else(|| EvalError::UnknownMacro {
					name: name.clone(),
					span: span.clone(),
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				macro_function(&arguments).map_err(|source| EvalError::CallError {
					source,
					span: span.clone(),
				})
			}
		}
	}
}
