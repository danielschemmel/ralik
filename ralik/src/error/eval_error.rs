use thiserror::Error;

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
