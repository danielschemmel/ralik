use proc_macro2::Span;

use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub enum Location {
	Spanned(Span),
}

impl From<Span> for Location {
	fn from(span: Span) -> Location {
		Location::Spanned(span)
	}
}

impl From<&Span> for Location {
	fn from(span: &Span) -> Location {
		Location::Spanned(*span)
	}
}

impl std::fmt::Display for Location {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Location::Spanned(span) => write!(
				f,
				"{}:{} to {}:{}",
				span.start().line,
				span.start().column,
				span.end().line,
				span.end().column
			),
		}
	}
}
