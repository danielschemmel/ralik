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

	#[error(transparent)]
	MissingVecGeneric(#[from] MissingVecGeneric),
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

#[derive(Error, Debug)]
#[error("The given context does not have a Vec generic registered")]
pub struct MissingVecGeneric;

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
