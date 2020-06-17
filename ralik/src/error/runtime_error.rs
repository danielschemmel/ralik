use thiserror::Error;

use crate::TypeHandle;

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

	#[error(transparent)]
	InvalidCoreType(#[from] InvalidCoreType),

	#[error("Could not create object")]
	ValueCreationError(#[from] ValueCreationError),

	#[error("Panic!")]
	Panic(#[from] anyhow::Error),
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

impl From<TupleStructCreationError> for RuntimeError {
	fn from(value: TupleStructCreationError) -> Self {
		RuntimeError::ValueCreationError(value.into())
	}
}

impl From<UnitStructCreationError> for RuntimeError {
	fn from(value: UnitStructCreationError) -> Self {
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

	#[error("Could not create object of tuple struct type")]
	TupleStructCreationError(#[from] TupleStructCreationError),

	#[error("Could not create object of unit struct type")]
	UnitStructCreationError(#[from] UnitStructCreationError),

	#[error("Could not create unit variant of enum type")]
	EnumUnitVariantCreationError(#[from] EnumUnitVariantCreationError),

	#[error("Could not create tuple variant of enum type")]
	EnumTupleVariantCreationError(#[from] EnumTupleVariantCreationError),

	#[error("Could not create struct variant of enum type")]
	EnumStructVariantCreationError(#[from] EnumStructVariantCreationError),

	#[error("Could not create object of array type")]
	ArrayCreationError(#[from] ArrayCreationError),
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

	#[error("Tuple type expects {} elements, but {} where provided", .type_element_count, .provided_element_count)]
	ElementCount {
		type_element_count: usize,
		provided_element_count: usize,
	},

	#[error("Element number {} should have type `{}`, but has type `{}`", .index, .expected.name(), .actual.name())]
	ElementTypeMismatch {
		index: usize,
		expected: TypeHandle,
		actual: TypeHandle,
	},
}

#[derive(Error, Debug)]
pub enum TupleStructCreationError {
	#[error("Type is not a valid tuple type")]
	InvalidType(#[from] InvalidTupleStructType),

	#[error("Tuple type expects {} elements, but {} where provided", .type_element_count, .provided_element_count)]
	ElementCount {
		type_element_count: usize,
		provided_element_count: usize,
	},

	#[error("Element number {} should have type `{}`, but has type `{}`", .index, .expected.name(), .actual.name())]
	ElementTypeMismatch {
		index: usize,
		expected: TypeHandle,
		actual: TypeHandle,
	},
}

#[derive(Error, Debug)]
pub enum StructCreationError {
	#[error("Type is not a valid struct type")]
	InvalidType(#[from] InvalidStructType),

	#[error("Missing field `{}` while creating object of type `{}`", .field_name, .r#type.name())]
	MissingField { r#type: TypeHandle, field_name: String },

	#[error("Duplicate field `{}` while creating object of type `{}`", .field_name, .r#type.name())]
	DuplicateField { r#type: TypeHandle, field_name: String },

	#[error("Superfluous field `{}` while creating object of type `{}`", .field_name, .r#type.name())]
	SuperfluousField { r#type: TypeHandle, field_name: String },

	#[error("Cannot initialize field `{}` with type `{}` for an object of type `{}` with a value of type `{}`", .field_name, .field_type.name(), .r#type.name(), value_type.name())]
	FieldTypeMismatch {
		r#type: TypeHandle,
		field_name: String,
		field_type: TypeHandle,
		value_type: TypeHandle,
	},
}

#[derive(Error, Debug)]
pub enum UnitStructCreationError {
	#[error("Type is not a valid struct type")]
	InvalidType(#[from] InvalidUnitStructType),

	#[error("Field `{}` provided while creating object of unit struct type `{}`", .field_name, .r#type.name())]
	FieldProvided { r#type: TypeHandle, field_name: String },
}

#[derive(Error, Debug)]
pub enum EnumUnitVariantCreationError {
	#[error("Type is not a valid struct type")]
	InvalidType(#[from] InvalidEnumType),

	#[error("Enum type `{}` does nut provide a variant `{}`", .r#type.name(), .variant_name)]
	VariantMissing { r#type: TypeHandle, variant_name: String },

	#[error("Variant `{}` of Enum type `{}` is not a unit variant", .variant_name, .r#type.name())]
	NotUnitVariant { r#type: TypeHandle, variant_name: String },

	#[error("Field `{}` provided while creating object of unit struct type `{}`", .field_name, .r#type.name())]
	FieldProvided { r#type: TypeHandle, field_name: String },
}

#[derive(Error, Debug)]
pub enum EnumTupleVariantCreationError {
	#[error("Type is not a valid tuple type")]
	InvalidType(#[from] InvalidEnumType),

	#[error("Enum type `{}` does nut provide a variant `{}`", .r#type.name(), .variant_name)]
	VariantMissing { r#type: TypeHandle, variant_name: String },

	#[error("Variant `{}` of Enum type `{}` is not a tuple variant", .variant_name, .r#type.name())]
	NotTupleVariant { r#type: TypeHandle, variant_name: String },

	#[error("Tuple type expects {} elements, but {} where provided", .type_element_count, .provided_element_count)]
	ElementCount {
		type_element_count: usize,
		provided_element_count: usize,
	},

	#[error("Element number {} should have type `{}`, but has type `{}`", .index, .expected.name(), .actual.name())]
	ElementTypeMismatch {
		index: usize,
		expected: TypeHandle,
		actual: TypeHandle,
	},
}

#[derive(Error, Debug)]
pub enum EnumStructVariantCreationError {
	#[error("Type is not a valid struct type")]
	InvalidType(#[from] InvalidEnumType),

	#[error("Enum type `{}` does nut provide a variant `{}`", .r#type.name(), .variant_name)]
	VariantMissing { r#type: TypeHandle, variant_name: String },

	#[error("Variant `{}` of Enum type `{}` is not a struct variant", .variant_name, .r#type.name())]
	NotStructVariant { r#type: TypeHandle, variant_name: String },

	#[error("Missing field `{}` while creating object of type `{}`", .field_name, .r#type.name())]
	MissingField { r#type: TypeHandle, field_name: String },

	#[error("Duplicate field `{}` while creating object of type `{}`", .field_name, .r#type.name())]
	DuplicateField { r#type: TypeHandle, field_name: String },

	#[error("Superfluous field `{}` while creating object of type `{}`", .field_name, .r#type.name())]
	SuperfluousField { r#type: TypeHandle, field_name: String },

	#[error("Cannot initialize field `{}` with type `{}` for an object of type `{}` with a value of type `{}`", .field_name, .field_type.name(), .r#type.name(), value_type.name())]
	FieldTypeMismatch {
		r#type: TypeHandle,
		field_name: String,
		field_type: TypeHandle,
		value_type: TypeHandle,
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
	InvalidBoolType(#[from] InvalidBoolType),
	InvalidIntegerType(#[from] InvalidIntegerType),
	InvalidCharType(#[from] InvalidCharType),
	InvalidStringType(#[from] InvalidStringType),
	InvalidTupleType(#[from] InvalidTupleType),
	InvalidArrayType(#[from] InvalidArrayType),
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
	#[error("The given context does not have the tuple type `{type_name}` registered")]
	Missing { type_name: String },

	#[error("The type `{}` does not have the kind `TypeKind::Tuple`", .r#type.name())]
	NotTupleType { r#type: TypeHandle },

	#[error("The given context does not have the type `{missing_element_type_name}` registered that is required to create the tuple `{make_tuple_name}`")]
	MissingSubtype {
		make_tuple_name: String,
		missing_element_type_name: String,
	},
}

#[derive(Error, Debug)]
pub enum InvalidTupleStructType {
	#[error("The given context does not have the tuple type `{type_name}` registered")]
	Missing { type_name: String },

	#[error("The type `{}` does not have the kind `TypeKind::TupleStruct`", .r#type.name())]
	NotTupleStructType { r#type: TypeHandle },
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
pub enum InvalidUnitStructType {
	#[error("The given context does not have the structure type `{type_name}` registered")]
	Missing { type_name: String },

	#[error("The type `{}` does not have the kind `TypeKind::UnitStruct`", .r#type.name())]
	NotUnitStructType { r#type: TypeHandle },
}

#[derive(Error, Debug)]
pub enum InvalidStructType {
	#[error("The given context does not have the structure type `{type_name}` registered")]
	Missing { type_name: String },

	#[error("The type `{}` does not have the kind `TypeKind::Struct`", .r#type.name())]
	NotStructType { r#type: TypeHandle },

	#[error("The type `{}` does not provide field names", .r#type.name())]
	NoFieldNames { r#type: TypeHandle },
}

#[derive(Error, Debug)]
pub enum InvalidEnumType {
	#[error("The given context does not have the structure type `{type_name}` registered")]
	Missing { type_name: String },

	#[error("The type `{}` does not have the kind `TypeKind::Enum`", .r#type.name())]
	NotEnumType { r#type: TypeHandle },
}
