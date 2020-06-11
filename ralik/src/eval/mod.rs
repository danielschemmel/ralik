use super::ast::{AtomicExpression, Expression};
use super::Value;

mod error;
pub use error::EvalError;

pub fn eval(expression: &Expression) -> Result<Value, EvalError> {
	unimplemented!()
}
