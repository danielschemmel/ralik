use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
	#[error("Could not parse input")]
	ParseError{
		#[from]
		cause: syn::Error,
		// backtrace: std::backtrace::Backtrace,
	},
}
