use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
	#[error("Could not parse input")]
	ParseError {
		#[from]
		cause: syn::Error,
		// backtrace: std::backtrace::Backtrace,
	},
	#[error("Could not evaluate input")]
	EvalError {
		#[from]
		cause: super::eval::EvalError,
		// backtrace: std::backtrace::Backtrace,
	},

	#[error(transparent)]
	Other {
		#[from]
		cause: anyhow::Error,
	},
}

#[derive(Error, Debug)]
#[error("Invalid core type")]
pub enum InvalidCoreType {
	InvalidBoolType(#[from] InvalidBoolType),
	InvalidCharType(#[from] InvalidCharType),
	InvalidIntegerType(#[from] InvalidIntegerType),
	InvalidStringType(#[from] InvalidStringType),
}

#[derive(Error, Debug)]
pub enum InvalidBoolType {
	#[error("The given context does not have a type `{}` registered", crate::types::BoolName)]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidCharType {
	#[error("The given context does not have a type `{}` registered", crate::types::CharName)]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidIntegerType {
	#[error("The given context does not have a type `{}` registered", crate::types::IntegerName)]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidStringType {
	#[error("The given context does not have a type `{}` registered", crate::types::StringName)]
	Missing,
}
