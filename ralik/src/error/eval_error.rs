use proc_macro2::Span;
use thiserror::Error;

use std::convert::TryInto;

use super::{Location, RuntimeError};

#[derive(Error, Debug)]
pub enum EvalError {
	#[error("Could not parse input at {}", Location::from(.cause.span()))]
	ParseError {
		#[from]
		cause: syn::Error,
	},

	#[error("Variable `{name}` does not exist when referenced at {at}")]
	UnknownVariable { name: String, at: Location },

	#[error("Function `{name}` does not exist when referenced at {at}")]
	UnknownFunction { name: String, at: Location },

	#[error("Member function `{name}` does not exist for type `{type_name}` when referenced at {at}")]
	UnknownMemberFunction {
		name: String,
		type_name: String,
		at: Location,
	},

	#[error("Macro `{name}!` does not exist when referenced at {at}")]
	UnknownMacro { name: String, at: Location },

	#[error("Member field `{member_name}` does not exist for objects of type `{type_name}` at {at}")]
	InvalidFieldAccess {
		member_name: String,
		type_name: String,
		at: Location,
	},

	#[error("Array literal has mixed types: Element {index_1} has type `{type_1}` and element {index_2} has type `{type_2}` (at {at})")]
	MixedArray {
		index_1: usize,
		type_1: String,
		index_2: usize,
		type_2: String,
		at: Location,
	},

	#[error("Operand to `&&` has type `{type_name}` (should be boolean) at {at}")]
	NotBoolInLazyAnd { type_name: String, at: Location },

	#[error("Operand to `||` has type `{type_name}` (should be boolean) at {at}")]
	NotBoolInLazyOr { type_name: String, at: Location },

	#[error("Failed to call free function `{name}` at {at}")]
	FunctionRuntimeError {
		name: String,
		#[source]
		source: RuntimeError,
		at: Location,
	},

	#[error("Failed to call macro `{name}!` at {at}")]
	MacroRuntimeError {
		name: String,
		#[source]
		source: RuntimeError,
		at: Location,
	},

	#[error("Failed to call member function `{name}` on object of type `{type_name}` at {at}")]
	MemberRuntimeError {
		name: String,
		type_name: String,
		#[source]
		source: RuntimeError,
		at: Location,
	},

	#[error("Could not create object at {at}")]
	ObjectCreationError {
		#[source]
		source: crate::error::ValueCreationError,
		at: Location,
	},

	#[error("Could not create object of core type at {at}")]
	InvalidCoreType {
		#[source]
		source: crate::error::InvalidCoreType,
		at: Location,
	},

	#[error("Expressions creating empty arrays are not currently supported.")]
	EmptyArray { at: Location },
}

impl EvalError {
	pub fn span(&self) -> Option<Span> {
		match self {
			EvalError::ParseError { cause: err } => Some(err.span()),
			EvalError::UnknownVariable { name: _, at: loc }
			| EvalError::UnknownFunction { name: _, at: loc }
			| EvalError::UnknownMemberFunction {
				name: _,
				type_name: _,
				at: loc,
			}
			| EvalError::UnknownMacro { name: _, at: loc }
			| EvalError::InvalidFieldAccess {
				member_name: _,
				type_name: _,
				at: loc,
			}
			| EvalError::MixedArray {
				index_1: _,
				type_1: _,
				index_2: _,
				type_2: _,
				at: loc,
			}
			| EvalError::NotBoolInLazyAnd { type_name: _, at: loc }
			| EvalError::NotBoolInLazyOr { type_name: _, at: loc }
			| EvalError::FunctionRuntimeError {
				name: _,
				source: _,
				at: loc,
			}
			| EvalError::MacroRuntimeError {
				name: _,
				source: _,
				at: loc,
			}
			| EvalError::MemberRuntimeError {
				name: _,
				type_name: _,
				source: _,
				at: loc,
			}
			| EvalError::ObjectCreationError { source: _, at: loc }
			| EvalError::InvalidCoreType { source: _, at: loc }
			| EvalError::EmptyArray { at: loc } => loc.try_into().ok(),
		}
	}
}
