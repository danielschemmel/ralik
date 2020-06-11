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
				let _value = expression.eval(context)?;
				match prefix {
					Prefix::Not(_span) => unimplemented!(),
					Prefix::Minus(_span) => unimplemented!(),
				}
			}
			Expression::Suffix(expression, suffix) => {
				let _value = expression.eval(context)?;
				match suffix {
					Suffix::Unwrap(_span) => unimplemented!(),
					Suffix::Field(_name, _span) => unimplemented!(),
					Suffix::TupleIndex(_index, _span) => unimplemented!(),
					Suffix::ArrayIndex(_index, _span) => unimplemented!(),
					Suffix::FunctionCall(_name, _name_span, _arguments, _arguments_span) => unimplemented!(),
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
				.get("$")
				.cloned()
				.ok_or_else(|| EvalError::InvalidVariableReference {
					name: "$".to_string(),
					span: span.clone(),
				}),
			AtomicExpression::FunctionCall(_name, _arguments, _span) => unimplemented!(),
			AtomicExpression::MacroCall(_name, _arguments, _span) => unimplemented!(),
		}
	}
}
