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
pub enum MissingBasicType {
	#[error(transparent)]
	MissingBoolType(#[from] MissingBoolType),

	#[error(transparent)]
	MissingCharType(#[from] MissingCharType),

	#[error(transparent)]
	MissingIntegerType(#[from] MissingIntegerType),

	#[error(transparent)]
	MissingStringType(#[from] MissingStringType),
}


#[derive(Error, Debug)]
#[error("The given context does not have a type `{}` registered", crate::types::BoolName)]
pub struct MissingBoolType;

#[derive(Error, Debug)]
#[error("The given context does not have a type `{}` registered", crate::types::CharName)]
pub struct MissingCharType;

#[derive(Error, Debug)]
#[error("The given context does not have a type `{}` registered", crate::types::IntegerName)]
pub struct MissingIntegerType;

#[derive(Error, Debug)]
#[error("The given context does not have a type `{}` registered", crate::types::StringName)]
pub struct MissingStringType;
