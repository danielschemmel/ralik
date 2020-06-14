#![forbid(unsafe_code)]

use proc_macro2::TokenStream;

pub mod error;

mod context;
pub use context::Context;

mod eval;

mod ops;

mod syntax;
use syntax::ast;

pub mod types;
pub use types::{Type, TypeHandle};

mod value;
pub use value::Value;

pub fn eval_str(source: &str, context: &Context) -> Result<Value, error::EvalError> {
	let expression = syn::parse_str::<syntax::ast::Expression>(source)?;
	eval_expression(expression, context)
}

pub fn eval_tokens(source: TokenStream, context: &Context) -> Result<Value, error::EvalError> {
	let expression = syn::parse2::<syntax::ast::Expression>(source)?;
	eval_expression(expression, context)
}

fn eval_expression(expression: syntax::ast::Expression, context: &Context) -> Result<Value, error::EvalError> {
	Ok(eval::Eval::eval(&expression, context)?)
}
