#![forbid(unsafe_code)]

mod error;
pub use error::ParseError;

mod eval;
mod syntax;
mod value;

pub fn run_str(source: &str) -> Result<syntax::ast::Expression, ParseError> {
	let expression = syn::parse_str::<syntax::ast::Expression>(source)?;
	Ok(expression)
}
