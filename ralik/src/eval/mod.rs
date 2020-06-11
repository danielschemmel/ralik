use super::ast::{AtomicExpression, Expression};
use super::{Context, Value};

mod error;
pub use error::EvalError;

pub trait Eval {
	fn eval(&self, context: &Context) -> Result<Value, EvalError>;
}

impl Eval for Expression {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		let value = self.atom.eval(context)?;

		for _suffix in &self.suffixes {
			unimplemented!();
		}

		Ok(value)
	}
}

impl Eval for AtomicExpression {
	fn eval(&self, context: &Context) -> Result<Value, EvalError> {
		match self {
			AtomicExpression::Dollar(span) => context
				.get("$")
				.cloned()
				.ok_or_else(|| EvalError::VariableReferenceError {
					name: "$".to_string(),
					span: span.clone(),
				}),
			_ => unimplemented!()
		}
	}
}
