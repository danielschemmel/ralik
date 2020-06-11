#![forbid(unsafe_code)]

use proc_macro2::TokenStream;

mod error;
pub use error::RunError;

mod context;
pub use context::Context;

mod eval;

mod syntax;
use syntax::ast;

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
