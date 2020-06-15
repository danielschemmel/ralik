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
			Location::Spanned(span) => {
				let (start, end) = (&span.start(), &span.end());
				if start.line == end.line && end.column - start.column <= 1 {
					write!(f, "{}:{}", start.line, start.column)
				} else {
					write!(f, "{}:{} to {}:{}", start.line, start.column, end.line, end.column)
				}
			}
		}
	}
}
