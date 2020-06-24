use std::convert::TryFrom;

use crate::error::EvalError;

use super::ast::{AtomicExpression, BinaryOperator, Expression, Prefix, Suffix};
use super::{Context, Value};

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
					Prefix::Not(span) => call_member_function_0(context, crate::ops::NOT, value, span),
					Prefix::Minus(span) => call_member_function_0(context, crate::ops::NEGATE, value, span),
				}
			}
			Expression::Suffix(expression, suffix) => {
				let value = expression.eval(context)?;
				match suffix {
					Suffix::Unwrap(span) => call_member_function_0(context, crate::ops::UNWRAP, value, span),
					Suffix::Field(name, span) => value.field(name).cloned().ok_or_else(|| EvalError::InvalidFieldAccess {
						member_name: name.clone(),
						type_name: (&*value.get_type().name()).into(),
						at: span.into(),
					}),
					Suffix::TupleIndex(index, span) => {
						let index = usize::try_from(*index).map_err(|_| EvalError::InvalidFieldAccess {
							member_name: index.to_string(),
							type_name: (&*value.get_type().name()).into(),
							at: span.into(),
						})?;
						value
							.tuple_field(index)
							.cloned()
							.ok_or_else(|| EvalError::InvalidFieldAccess {
								member_name: index.to_string(),
								type_name: (&*value.get_type().name()).into(),
								at: span.into(),
							})
					}
					Suffix::ArrayIndex(index, span) => call_member_function_1(context, crate::ops::INDEX, value, index, span),
					Suffix::FunctionCall(name, name_span, arguments, _arguments_span) => {
						call_member_function_n(context, name, value, &arguments.arguments, name_span)
					}
				}
			}
			Expression::Binary(lhs, rhs, op) => {
				let lhs_value = lhs.eval(context)?;
				match op {
					BinaryOperator::Div(span) => call_member_function_1(context, crate::ops::DIV, lhs_value, rhs, span),
					BinaryOperator::Mul(span) => call_member_function_1(context, crate::ops::MUL, lhs_value, rhs, span),
					BinaryOperator::Rem(span) => call_member_function_1(context, crate::ops::REM, lhs_value, rhs, span),
					BinaryOperator::Add(span) => call_member_function_1(context, crate::ops::ADD, lhs_value, rhs, span),
					BinaryOperator::Sub(span) => call_member_function_1(context, crate::ops::SUB, lhs_value, rhs, span),
					BinaryOperator::Shl(span) => call_member_function_1(context, crate::ops::SHL, lhs_value, rhs, span),
					BinaryOperator::Shr(span) => call_member_function_1(context, crate::ops::SHR, lhs_value, rhs, span),
					BinaryOperator::BitAnd(span) => call_member_function_1(context, crate::ops::BIT_AND, lhs_value, rhs, span),
					BinaryOperator::BitXor(span) => call_member_function_1(context, crate::ops::BIT_XOR, lhs_value, rhs, span),
					BinaryOperator::BitOr(span) => call_member_function_1(context, crate::ops::BIT_OR, lhs_value, rhs, span),
					BinaryOperator::Equal(span) => call_member_function_1(context, crate::ops::EQUAL, lhs_value, rhs, span),
					BinaryOperator::NotEqual(span) => {
						call_member_function_1(context, crate::ops::NOT_EQUAL, lhs_value, rhs, span)
					}
					BinaryOperator::Less(span) => call_member_function_1(context, crate::ops::LESS, lhs_value, rhs, span),
					BinaryOperator::LessOrEqual(span) => {
						call_member_function_1(context, crate::ops::LESS_OR_EQUAL, lhs_value, rhs, span)
					}
					BinaryOperator::Greater(span) => call_member_function_1(context, crate::ops::GREATER, lhs_value, rhs, span),
					BinaryOperator::GreaterOrEqual(span) => {
						call_member_function_1(context, crate::ops::GREATER_OR_EQUAL, lhs_value, rhs, span)
					}
					BinaryOperator::LazyAnd(span) => {
						let bool_type = context.get_bool_type().map_err(|err| EvalError::InvalidCoreType {
							source: err.into(),
							at: span.into(),
						})?;

						if !lhs_value.has_type(&bool_type) {
							return Err(EvalError::NotBoolInLazyAnd {
								type_name: (&*lhs_value.get_type().name()).into(),
								at: span.into(), // TODO: use the lhs span instead of the operator span here
							});
						}

						let lhs_bool = lhs_value.as_bool().unwrap();
						if lhs_bool == false {
							Ok(lhs_value)
						} else {
							let rhs_value = rhs.eval(context)?;
							if rhs_value.has_type(&bool_type) {
								Ok(rhs_value)
							} else {
								Err(EvalError::NotBoolInLazyAnd {
									type_name: (&*rhs_value.get_type().name()).into(),
									at: span.into(), // TODO: use the lhs span instead of the operator span here
								})
							}
						}
					}
					BinaryOperator::LazyOr(span) => {
						let bool_type = context.get_bool_type().map_err(|err| EvalError::InvalidCoreType {
							source: err.into(),
							at: span.into(),
						})?;

						if !lhs_value.has_type(&bool_type) {
							return Err(EvalError::NotBoolInLazyAnd {
								type_name: (&*lhs_value.get_type().name()).into(),
								at: span.into(), // TODO: use the lhs span instead of the operator span here
							});
						}

						let lhs_bool = lhs_value.as_bool().unwrap();
						if lhs_bool == true {
							Ok(lhs_value)
						} else {
							let rhs_value = rhs.eval(context)?;
							if rhs_value.has_type(&bool_type) {
								Ok(rhs_value)
							} else {
								Err(EvalError::NotBoolInLazyAnd {
									type_name: (&*rhs_value.get_type().name()).into(),
									at: span.into(), // TODO: use the lhs span instead of the operator span here
								})
							}
						}
					}
				}
			}
			Expression::Block(_) => unimplemented!(),
			Expression::If(_, _, _) => unimplemented!(),
			Expression::Else(_, _, _) => unimplemented!(),
			Expression::While(_, _, _) => unimplemented!(),
			Expression::Loop(_, _, _) => unimplemented!(),
		}
	}
}

impl Eval for AtomicExpression {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		match self {
			AtomicExpression::Unit(span) => Value::new_unit(context).map_err(|err| EvalError::ObjectCreationError {
				source: err.into(),
				at: span.into(),
			}),
			AtomicExpression::Parenthesized(expression, _span) => expression.eval(context),
			AtomicExpression::Tuple(expressions, span) => {
				let values = expressions
					.iter()
					.map(|expression| expression.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				Value::new_tuple(context, values).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::Array(expressions, span) => {
				let values = expressions
					.iter()
					.map(|expression| expression.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;

				if values.is_empty() {
					return Err(EvalError::EmptyArray { at: span.into() });
				}

				let type_0 = values[0].get_type();
				if let Some((index, value)) = values[1..]
					.iter()
					.enumerate()
					.find(|(_index, value)| !value.has_type(type_0))
				{
					return Err(EvalError::MixedArray {
						index_1: 0,
						type_1: (&*type_0.name()).to_owned(),
						index_2: index + 1,
						type_2: (&*value.get_type().name()).to_owned(),
						at: span.into(),
					});
				}

				Value::new_array(context, &type_0.clone(), values).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::LitBool(value, span) => {
				Value::new_bool(context, *value).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::LitInt(value, span) => {
				Value::new_integer(context, value.clone()).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::LitByte(value, span) => {
				Value::new_integer(context, *value).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::LitByteStr(value, span) => {
				let integer_type = context
					.get_integer_type()
					.map_err(|err| EvalError::ObjectCreationError {
						source: crate::error::IntegerCreationError::from(err).into(),
						at: span.into(),
					})?;
				let values = value
					.iter()
					.map(|byte| Value::new_integer(context, *byte))
					.collect::<Result<Vec<Value>, crate::error::IntegerCreationError>>()
					.map_err(|err| EvalError::ObjectCreationError {
						source: err.into(),
						at: span.into(),
					})?;
				Value::new_array(context, &integer_type, values).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::LitChar(value, span) => {
				Value::new_char(context, *value).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::LitStr(value, span) => {
				Value::new_string(context, value.clone().into_boxed_str()).map_err(|err| EvalError::ObjectCreationError {
					source: err.into(),
					at: span.into(),
				})
			}
			AtomicExpression::Dollar(span) => context.get_variable("$").ok_or_else(|| EvalError::UnknownVariable {
				name: "$".into(),
				at: span.into(),
			}),
			AtomicExpression::FunctionCall(name, name_span, arguments, _arguments_span) => {
				let function = context.get_function(name).ok_or_else(|| EvalError::UnknownFunction {
					name: name.clone(),
					at: name_span.into(),
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				function(context, &arguments).map_err(|source| EvalError::FunctionRuntimeError {
					name: name.into(),
					source,
					at: name_span.into(),
				})
			}
			AtomicExpression::MacroCall(name, name_span, arguments, _arguments_span) => {
				let macro_function = context.get_macro(name).ok_or_else(|| EvalError::UnknownMacro {
					name: name.clone(),
					at: name_span.into(),
				})?;
				let arguments = arguments
					.arguments
					.iter()
					.map(|argument| argument.eval(context))
					.collect::<Result<Vec<Value>, EvalError>>()?;
				macro_function(context, &arguments).map_err(|source| EvalError::MacroRuntimeError {
					name: name.into(),
					source,
					at: name_span.into(),
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
