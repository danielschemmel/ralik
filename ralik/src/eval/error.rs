use proc_macro2::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
	#[error("Variable `{name}` does not exist when referenced at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	UnknownVariable {
		name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},
	#[error("Function `{name}` does not exist when referenced at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	UnknownFunction {
		name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},
	#[error("Macro `{name}!` does not exist when referenced at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	UnknownMacro {
		name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},
	#[error("Member field `{member_name}` does not exist for objects of type `{type_name}` at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	InvalidFieldAccess {
		member_name: String,
		type_name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},
	#[error("{source} at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	CallError { source: CallError, span: Span },
}

#[derive(Error, Debug)]
pub enum CallError {
	#[error("Member function `{member_name}` does not exist for objects of type `{type_name}`")]
	FunctionDoesNotExist {
		member_name: String,
		type_name: String,
		// backtrace: std::backtrace::Backtrace,
	},
}
