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
pub enum InvalidBasicType {
	#[error(transparent)]
	MissingBoolType(#[from] InvalidBoolType),

	#[error(transparent)]
	MissingCharType(#[from] InvalidCharType),

	#[error(transparent)]
	MissingIntegerType(#[from] InvalidIntegerType),

	#[error(transparent)]
	MissingStringType(#[from] InvalidStringType),

	#[error(transparent)]
	MissingVecGeneric(#[from] InvalidVecGeneric),
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

#[derive(Error, Debug)]
pub enum InvalidVecGeneric {
	#[error("The given context does not have a type `{}` registered", crate::types::VecName)]
	Missing,
}

#[derive(Error, Debug)]
pub enum GenericError {
	#[error("Failed to construct generic type from generic `{name}`")]
	GenericCallError {
		name: String,
		#[source]
		source: GenericCallError,
	},

	#[error("Generic `{name}` is not registered with the given context")]
	GenericMissing { name: String },
}
#[derive(Error, Debug)]
pub enum GenericCallError {}
