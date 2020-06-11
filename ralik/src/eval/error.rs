use proc_macro2::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
	#[error("Variable {name} does not exist when referenced at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	VariableReferenceError {
		name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},
}
