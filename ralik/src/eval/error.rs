use proc_macro2::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
	#[error("Variable {name} does not exist when referenced at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	InvalidVariableReference {
		name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},
}

#[derive(Error, Debug)]
pub enum CallError {
	#[error("Member function {name} does not exist for objects of type {type_name}")]
	FunctionDoesNotExist {
		name: String,
		type_name: String,
		// backtrace: std::backtrace::Backtrace,
	},
}
