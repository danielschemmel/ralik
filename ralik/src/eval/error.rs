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

	#[error("Member function `{name}` does not exist for type `{type_name}` when referenced at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	UnknownMemberFunction {
		name: String,
		type_name: String,
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

	#[error("Operand to `&&` has type `{type_name}` (should be boolean) at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	NotBoolInLazyAnd {
		type_name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},

	#[error("Operand to `||` has type `{type_name}` (should be boolean) at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	NotBoolInLazyOr {
		type_name: String,
		span: Span,
		// backtrace: std::backtrace::Backtrace,
	},

	#[error("Failed to call free function `{name}` at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	FunctionCallError {
		name: String,
		#[source]
		source: CallError,
		span: Span,
	},

	#[error("Failed to call macro `{name}!` at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	MacroCallError {
		name: String,
		#[source]
		source: CallError,
		span: Span,
	},

	#[error("Failed to call member function `{name}` on object of type `{type_name}` at {}:{} to {}:{}", span.start().line, span.start().column, span.end().line, span.end().column)]
	MemberCallError {
		name: String,
		type_name: String,
		#[source]
		source: CallError,
		span: Span,
	},

	#[error("The given context does not have a type `{}` available to create a new object at {}:{} to {}:{}", crate::types::BoolName, span.start().line, span.start().column, span.end().line, span.end().column)]
	MissingBoolType { span: Span },

	#[error("The given context does not have a type `{}` available to create a new object at {}:{} to {}:{}", crate::types::CharName, span.start().line, span.start().column, span.end().line, span.end().column)]
	MissingCharType { span: Span },

	#[error("The given context does not have a type `{}` available to create a new object at {}:{} to {}:{}", crate::types::IntegerName, span.start().line, span.start().column, span.end().line, span.end().column)]
	MissingIntegerType { span: Span },

	#[error("The given context does not have a type `{}` available to create a new object at {}:{} to {}:{}", crate::types::StringName, span.start().line, span.start().column, span.end().line, span.end().column)]
	MissingStringType { span: Span },
}

#[derive(Error, Debug)]
pub enum CallError {
	#[error(
		"Argument {argument_number} has type {actual_type_name}, but type {expected_type_name} was expected instead"
	)]
	InvalidArgumentType {
		argument_number: usize,
		actual_type_name: String,
		expected_type_name: String,
	},

	#[error("Invalid number of arguments: {actual} (expected {expected} arguments, including `self`)")]
	InvalidNumberOfArguments { actual: usize, expected: usize },

	#[error("An operation overflowed")]
	Overflow(#[from] Overflow),

	#[error(transparent)]
	MissingBoolType(#[from] crate::error::InvalidBoolType),

	#[error(transparent)]
	MissingCharType(#[from] crate::error::InvalidCharType),

	#[error(transparent)]
	MissingIntegerType(#[from] crate::error::InvalidIntegerType),

	#[error(transparent)]
	MissingStringType(#[from] crate::error::InvalidStringType),

	#[error(transparent)]
	Other {
		#[from]
		cause: anyhow::Error,
	},
}

#[derive(Error, Debug)]
pub enum Overflow {
	#[error("Negative shifts overflow by definition")]
	NegativeShift,

	#[error("Attempt to shift by an amount that is to large")]
	LargeShift,

	#[error("Value does not fit into a u32")]
	U32,

	#[error("Value does not fit into a usize")]
	USize,
}
