use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
	#[error("Could not parse input")]
	ParseError{
		#[from]
		cause: syn::Error,
		// backtrace: std::backtrace::Backtrace,
	},
}
