#![forbid(unsafe_code)]

use proc_macro2::TokenStream;

mod error;
pub use error::RunError;

mod eval;

mod syntax;
use syntax::ast;

mod value;

fn run_expression(expression: syntax::ast::Expression) -> Result<syntax::ast::Expression, RunError> {
	Ok(expression)
}

pub fn run_str(source: &str) -> Result<syntax::ast::Expression, RunError> {
	let expression = syn::parse_str::<syntax::ast::Expression>(source)?;
	run_expression(expression)
}

pub fn run_tokens(source: TokenStream) -> Result<syntax::ast::Expression, RunError> {
	let expression = syn::parse2::<syntax::ast::Expression>(source)?;
	run_expression(expression)
}
