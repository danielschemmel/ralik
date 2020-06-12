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
