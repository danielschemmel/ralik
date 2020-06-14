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

	#[error("Could not create object")]
	ValueCreationError(#[from] ValueCreationError),

	#[error("Panic!")]
	Panic(#[from] anyhow::Error),
}

impl From<UnitCreationError> for RuntimeError {
	fn from(value: UnitCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<BoolCreationError> for RuntimeError {
	fn from(value: BoolCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<IntegerCreationError> for RuntimeError {
	fn from(value: IntegerCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<CharCreationError> for RuntimeError {
	fn from(value: CharCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<StringCreationError> for RuntimeError {
	fn from(value: StringCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<TupleCreationError> for RuntimeError {
	fn from(value: TupleCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<StructCreationError> for RuntimeError {
	fn from(value: StructCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<ArrayCreationError> for RuntimeError {
	fn from(value: ArrayCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
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
pub enum ValueCreationError {
	#[error("Could not create object of type `{}`", crate::types::unit_name())]
	UnitCreationError(#[from] UnitCreationError),

	#[error("Could not create object of type `{}`", crate::types::bool_name())]
	BoolCreationError(#[from] BoolCreationError),

	#[error("Could not create object of type `{}`", crate::types::integer_name())]
	IntegerCreationError(#[from] IntegerCreationError),

	#[error("Could not create object of type `{}`", crate::types::char_name())]
	CharCreationError(#[from] CharCreationError),

	#[error("Could not create object of type `{}`", crate::types::string_name())]
	StringCreationError(#[from] StringCreationError),

	#[error("Could not create object of tuple type")]
	TupleCreationError(#[from] TupleCreationError),

	#[error("Could not create object of struct type")]
	StructCreationError(#[from] StructCreationError),

	#[error("Could not create object of array type")]
	ArrayCreationError(#[from] ArrayCreationError),
}

#[derive(Error, Debug)]
pub enum UnitCreationError {
	#[error("Core type `{}` (Unit) is invalid", crate::types::unit_name())]
	InvalidType(#[from] InvalidUnitType),
}

#[derive(Error, Debug)]
pub enum BoolCreationError {
	#[error("Core type `{}` is invalid", crate::types::bool_name())]
	InvalidType(#[from] InvalidBoolType),
}

#[derive(Error, Debug)]
pub enum IntegerCreationError {
	#[error("Core type `{}` is invalid", crate::types::integer_name())]
	InvalidType(#[from] InvalidIntegerType),
}

#[derive(Error, Debug)]
pub enum CharCreationError {
	#[error("Core type `{}` is invalid", crate::types::char_name())]
	InvalidType(#[from] InvalidCharType),
}

#[derive(Error, Debug)]
pub enum StringCreationError {
	#[error("Core type `{}` is invalid", crate::types::string_name())]
	InvalidType(#[from] InvalidStringType),
}

#[derive(Error, Debug)]
pub enum TupleCreationError {
	#[error("Type is not a valid tuple type")]
	InvalidType(#[from] InvalidTupleType),
}

#[derive(Error, Debug)]
pub enum StructCreationError {
	#[error("Type is not a valid struct type")]
	InvalidType(#[from] InvalidStructType),

	#[error("Missing field `{field_name}` while creating object of type `{type_name}`")]
	MissingField { type_name: String, field_name: String },

	#[error("Superfluous field `{field_name}` while creating object of type `{type_name}`")]
	SuperfluousField { type_name: String, field_name: String },

	#[error("Cannot initialize field `{field_name}` with type `{field_type_name}` for an object of type `{type_name}` with a value of type `{value_type_name}`")]
	FieldTypeMismatch {
		type_name: String,
		field_name: String,
		field_type_name: String,
		value_type_name: String,
	},
}

#[derive(Error, Debug)]
pub enum ArrayCreationError {
	#[error("Type is not a valid array type")]
	InvalidType(#[from] InvalidArrayType),
}

#[derive(Error, Debug)]
#[error("Invalid core type")]
pub enum InvalidCoreType {
	InvalidUnitType(#[from] InvalidUnitType),
	InvalidBoolType(#[from] InvalidBoolType),
	InvalidIntegerType(#[from] InvalidIntegerType),
	InvalidCharType(#[from] InvalidCharType),
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
	#[error(
		"The given context does not have a type `{}` registered",
		crate::types::integer_name()
	)]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidStringType {
	#[error(
		"The given context does not have a type `{}` registered",
		crate::types::string_name()
	)]
	Missing,
}

#[derive(Error, Debug)]
pub enum InvalidTupleType {
	#[error("The type `{type_name}` is not a tuple type")]
	NotTupleType { type_name: String },

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
	MissingSubtype { element_type_name: String },

	#[error("The value `{value:?}` does not have the right type to be used in an array of type `{type_name}` (error occurred at index {index})")]
	InvalidElement {
		value: crate::Value,
		index: usize,
		type_name: String,
	},
}

#[derive(Error, Debug)]
pub enum InvalidStructType {
	#[error("The given context does not have the structure type `{type_name}` registered")]
	Missing { type_name: String },

	#[error("The type `{type_name}` is not a struct type")]
	NotStructType { type_name: String },
}
