use super::ast::{AtomicExpression, BinaryOperator, Expression, Prefix, Suffix};
use super::{Context, Value};

mod error;
pub use error::{CallError, EvalError, Overflow};

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
					Prefix::Not(span) => call_member_function_0(crate::ops::NOT, value, span),
					Prefix::Minus(span) => call_member_function_0(crate::ops::NEGATE, value, span),
				}
			}
			Expression::Suffix(expression, suffix) => {
				let value = expression.eval(context)?;
				match suffix {
					Suffix::Unwrap(span) => call_member_function_0(crate::ops::UNWRAP, value, span),
					Suffix::Field(name, span) => value.field(name).cloned().ok_or_else(|| EvalError::InvalidFieldAccess {
						member_name: name.clone(),
						type_name: value.get_type().name().to_string(),
						span: *span,
					}),
					Suffix::TupleIndex(index, span) => {
						let name = index.to_string();
						value
							.field(&name)
							.cloned()
							.ok_or_else(|| EvalError::InvalidFieldAccess {
								member_name: name,
								type_name: value.get_type().name().to_string(),
								span: *span,
							})
					}
					Suffix::ArrayIndex(index, span) => call_member_function_1(crate::ops::INDEX, value, index, span, context),
					Suffix::FunctionCall(name, name_span, arguments, _arguments_span) => {
						call_member_function_n(name, value, &arguments.arguments, name_span, context)
					}
				}
			}
			Expression::Binary(lhs, rhs, op) => {
				let lhs_value = lhs.eval(context)?;
				match op {
					BinaryOperator::Div(span) => call_member_function_1(crate::ops::DIV, lhs_value, rhs, span, context),
					BinaryOperator::Mul(span) => call_member_function_1(crate::ops::MUL, lhs_value, rhs, span, context),
					BinaryOperator::Rem(span) => call_member_function_1(crate::ops::REM, lhs_value, rhs, span, context),
					BinaryOperator::Add(span) => call_member_function_1(crate::ops::ADD, lhs_value, rhs, span, context),
					BinaryOperator::Sub(span) => call_member_function_1(crate::ops::SUB, lhs_value, rhs, span, context),
					BinaryOperator::Shl(span) => call_member_function_1(crate::ops::SHL, lhs_value, rhs, span, context),
					BinaryOperator::Shr(span) => call_member_function_1(crate::ops::SHR, lhs_value, rhs, span, context),
					BinaryOperator::BitAnd(span) => call_member_function_1(crate::ops::BIT_AND, lhs_value, rhs, span, context),
					BinaryOperator::BitXor(span) => call_member_function_1(crate::ops::BIT_XOR, lhs_value, rhs, span, context),
					BinaryOperator::BitOr(span) => call_member_function_1(crate::ops::BIT_OR, lhs_value, rhs, span, context),
					BinaryOperator::Equal(span) => call_member_function_1(crate::ops::EQUAL, lhs_value, rhs, span, context),
					BinaryOperator::NotEqual(span) => {
						call_member_function_1(crate::ops::NOT_EQUAL, lhs_value, rhs, span, context)
					}
					BinaryOperator::Less(span) => call_member_function_1(crate::ops::LESS, lhs_value, rhs, span, context),
					BinaryOperator::LessOrEqual(span) => {
						call_member_function_1(crate::ops::LESS_OR_EQUAL, lhs_value, rhs, span, context)
					}
					BinaryOperator::Greater(span) => call_member_function_1(crate::ops::GREATER, lhs_value, rhs, span, context),
					BinaryOperator::GreaterOrEqual(span) => {
						call_member_function_1(crate::ops::GREATER_OR_EQUAL, lhs_value, rhs, span, context)
					}
					BinaryOperator::LazyAnd(span) => {
						let lhs_bool = lhs_value.as_bool().ok_or_else(|| EvalError::NotBoolInLazyAnd {
							type_name: lhs_value.get_type().name().to_string(),
							span: *span, // TODO: use the lhs span instead of the operator span here
						})?;
						if lhs_bool == false {
							Ok(lhs_value)
						} else {
							let rhs_value = rhs.eval(context)?;
							if rhs_value.is_bool() {
								Ok(rhs_value)
							} else {
								Err(EvalError::NotBoolInLazyAnd {
									type_name: rhs_value.get_type().name().to_string(),
									span: *span, // TODO: use the lhs span instead of the operator span here
								})
							}
						}
					}
					BinaryOperator::LazyOr(span) => {
						let lhs_bool = lhs_value.as_bool().ok_or_else(|| EvalError::NotBoolInLazyAnd {
							type_name: lhs_value.get_type().name().to_string(),
							span: *span, // TODO: use the lhs span instead of the operator span here
						})?;
						if lhs_bool == true {
							Ok(lhs_value)
						} else {
							let rhs_value = rhs.eval(context)?;
							if rhs_value.is_bool() {
								Ok(rhs_value)
							} else {
								Err(EvalError::NotBoolInLazyAnd {
									type_name: rhs_value.get_type().name().to_string(),
									span: *span, // TODO: use the lhs span instead of the operator span here
								})
							}
						}
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
					span: *span,
				}),
			AtomicExpression::FunctionCall(name, name_span, arguments, _arguments_span) => {
				let function = context.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
					name: name.clone(),
					span: *name_span,
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				function(&arguments).map_err(|source| EvalError::FunctionCallError {
					name: name.to_string(),
					source,
					span: *name_span,
				})
			}
			AtomicExpression::MacroCall(name, name_span, arguments, _arguments_span) => {
				let macro_function = context.get_macro(name).ok_or_else(|| EvalError::UnknownMacro {
					name: name.clone(),
					span: *name_span,
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				macro_function(&arguments).map_err(|source| EvalError::MacroCallError {
					name: format!("{}!", name),
					source,
					span: *name_span,
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
