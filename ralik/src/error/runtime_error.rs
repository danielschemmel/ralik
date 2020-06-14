use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
	#[error(
		"Argument {argument_number} has type `{actual_type_name}`, but type `{expected_type_name}` was expected instead"
	)]
	InvalidArgumentType {
		argument_number: usize,
		actual_type_name: String,
		expected_type_name: String,
	},

	#[error("Invalid number of arguments: {actual} (expected {expected} arguments, including `self`)")]
	InvalidNumberOfArguments { actual: usize, expected: usize },

	#[error("Access out of bounds at {index} (len was {len})")]
	OutOfBounds { index: usize, len: usize },

	#[error("An operation overflowed")]
	Overflow(#[from] Overflow),

	#[error("A core type did not meet expectations")]
	InvalidCoreType(InvalidCoreType),

	#[error("Panic!")]
	Panic(#[from] anyhow::Error),
}

impl<T: Into<InvalidCoreType>> From<T> for RuntimeError {
	fn from(value: T) -> Self {
		value.into().into()
	}
}

#[derive(Error, Debug)]
pub enum Overflow {
	#[error("Negative shifts overflow by definition")]
	NegativeShift,

	#[error("Attempt to shift by an amount that is to large")]
	LargeShift,

	#[error("Value does not fit into a u32")]
	U32,

	#[error("Value does not fit into a usize")]
	USize,
}

#[derive(Error, Debug)]
#[error("Invalid core type")]
pub enum InvalidCoreType {
	InvalidUnitType(#[from] InvalidUnitType),
	InvalidBoolType(#[from] InvalidBoolType),
	InvalidCharType(#[from] InvalidCharType),
	InvalidIntegerType(#[from] InvalidIntegerType),
	InvalidStringType(#[from] InvalidStringType),
	InvalidTupleType(#[from] InvalidTupleType),
	InvalidArrayType(#[from] InvalidArrayType),
}

#[derive(Error, Debug)]
pub enum InvalidUnitType {
	#[error("The given context does not have a type `()` registered")]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidBoolType {
	#[error("The given context does not have a type `{}` registered", crate::types::bool_name())]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidCharType {
	#[error("The given context does not have a type `{}` registered", crate::types::char_name())]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidIntegerType {
	#[error("The given context does not have a type `{}` registered", crate::types::integer_name())]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidStringType {
	#[error("The given context does not have a type `{}` registered", crate::types::string_name())]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidTupleType {
	#[error("The given context does not have the type `{missing_element_type_name}` registered that is required to create the tuple `{tuple_name}`")]
	MissingSubtype {
		tuple_name: String,
		missing_element_type_name: String,
	},

	#[error(
		"Cannot create a tuple without any elements (note: the unit type `{}` is not a tuple)",
		crate::types::unit_name()
	)]
	ZeroElements,
}

#[derive(Error, Debug)]
pub enum InvalidArrayType {
	#[error("The given context does not have the type `{element_type_name}` registered to make an array out of")]
	MissingSubtype {
		element_type_name: String
	},

	#[error("The value `{value:?}` does not have the right type to be used in an array of type `{type_name}` (error occurred at index {index})")]
	InvalidElement {
		value: crate::Value,
		index: usize,
		type_name: String,
	}
}
