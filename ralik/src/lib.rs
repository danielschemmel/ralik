#![forbid(unsafe_code)]

use proc_macro2::TokenStream;

mod error;
pub use error::{
	GenericCallError, GenericError, InvalidBasicType, InvalidBoolType, InvalidCharType, InvalidIntegerType,
	InvalidStringType, InvalidVecGeneric, RunError,
};

mod context;
pub use context::Context;

mod eval;
pub use eval::CallError;

mod ops;

mod syntax;
use syntax::ast;

mod types;
pub use types::Type;

mod value;
pub use value::Value;

pub fn eval_str(source: &str, context: &Context) -> Result<Value, RunError> {
	let expression = syn::parse_str::<syntax::ast::Expression>(source)?;
	eval_expression(expression, context)
}

pub fn eval_tokens(source: TokenStream, context: &Context) -> Result<Value, RunError> {
	let expression = syn::parse2::<syntax::ast::Expression>(source)?;
	eval_expression(expression, context)
}

fn eval_expression(expression: syntax::ast::Expression, context: &Context) -> Result<Value, RunError> {
	Ok(eval::Eval::eval(&expression, context)?)
}
