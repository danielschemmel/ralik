#![forbid(unsafe_code)]

mod error;
pub use error::RunError;

mod eval;
mod syntax;
mod value;

pub fn run_str(source: &str) -> Result<syntax::ast::Expression, RunError> {
	let expression = syn::parse_str::<syntax::ast::Expression>(source)?;
	Ok(expression)
}
