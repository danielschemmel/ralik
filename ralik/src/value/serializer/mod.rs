use my_serde::ser;
use thiserror::Error;

use crate::error::InvalidStringType;
use crate::types::{TypeKind, Variant};
use crate::{Context, TypeHandle, Value};

mod maps;
use maps::SerializeMap;

mod sequences;
use sequences::SerializeSequence;

mod struct_variant;
use struct_variant::SerializeStructVariant;

mod tuple_variant;
use tuple_variant::SerializeTupleVariant;

impl Value {
	pub fn from_serde<T: ser::Serialize>(context: &Context, value: T, type_name: &str) -> Result<Self, SerializerError> {
		let r#type = context
			.get_type(type_name)
			.ok_or_else(|| SerializerError::MissingType {
				type_name: type_name.into(),
			})?;
		Self::from_serde_by_type(context, value, r#type)
	}

	fn from_serde_by_type<T: ser::Serialize>(
		context: &Context,
		value: T,
		r#type: TypeHandle,
	) -> Result<Self, SerializerError> {
		value.serialize(Serializer {
			context,
			expected_type: r#type,
		})
	}
}

#[derive(Error, Debug)]
pub enum SerializerError {
	#[error("Context does not contain the requested type `{type_name}`")]
	MissingType { type_name: String },

	#[error("Type mismatch: Expected type `{}`, but got `{}`", .expected.name(), .actual.name())]
	TypeMismatch { expected: TypeHandle, actual: TypeHandle },

	#[error("Type mismatch: Expected type `{}`, but serialized a type name `{}`", .expected.name(), .actual)]
	TypeNameMismatch { expected: TypeHandle, actual: String },

	#[error("Type mismatch: The expected type `{}` cannot be used to serialize a sequence", .expected.name())]
	InvalidTypeForSequence { expected: TypeHandle },

	#[error("Type mismatch: The expected type `{}` cannot be used to serialize a map", .expected.name())]
	InvalidTypeForMap { expected: TypeHandle },

	#[error("Type mismatch: The expected type `{}` cannot be used to serialize a sequence", .expected.name())]
	InvalidTypeForTuple { expected: TypeHandle },

	#[error("Type mismatch: The expected type `{}` cannot be used to serialize a variant", .expected.name())]
	InvalidTypeForVariant { expected: TypeHandle },

	#[error("Type mismatch: The expected type `{}` cannot be used to serialize a string", .expected.name())]
	InvalidTypeForString { expected: TypeHandle },

	#[error("Type mismatch: The expected type `{}` does not contain a variant named `{}`", .expected.name(), .variant_name)]
	InvalidVariant { expected: TypeHandle, variant_name: String },

	#[error("Variant mismatch: The enum variant `{}::{}` has the wrong kind", .r#type.name(), .variant_name)]
	VariantMismatch { r#type: TypeHandle, variant_name: String },

	#[error("Cannot instantiate string type for use as key")]
	InvalidStringTypeForKey(#[from] InvalidStringType),

	#[error("Floating point numbers are (currently?) not supported by RALIK")]
	Float,

	#[error("Encountered too many values for type `{}`", .r#type.name())]
	TooManyValues { r#type: TypeHandle },

	#[error("Constructing an object of type `{}` requires an additional {} values", .r#type.name(), .count)]
	TooFewValues { r#type: TypeHandle, count: usize },

	#[error("Encountered an unexpected key `{}` while constructing an object of type `{}`", .key, .r#type.name())]
	UnexpectedKey { r#type: TypeHandle, key: String },

	#[error("Encountered the key `{}` multiple times while constructing an object of type `{}`", .key, .r#type.name())]
	DuplicateKey { r#type: TypeHandle, key: String },

	#[error(transparent)]
	ValueCreationError(crate::error::ValueCreationError),

	#[error("Custom Error: {0}")]
	Custom(String),
}

impl<T: Into<crate::error::ValueCreationError>> From<T> for SerializerError {
	fn from(value: T) -> Self {
		SerializerError::ValueCreationError(value.into())
	}
}

impl ser::Error for SerializerError {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		SerializerError::Custom(msg.to_string())
	}
}

struct Serializer<'a> {
	context: &'a Context,
	expected_type: TypeHandle,
}

impl<'a> Serializer<'a> {
	fn expect_typed_value(self, value: Value) -> Result<Value, SerializerError> {
		if value.has_type(&self.expected_type) {
			Ok(value)
		} else {
			Err(SerializerError::TypeMismatch {
				expected: self.expected_type,
				actual: value.get_type().clone(),
			})
		}
	}
}

impl<'a> ser::Serializer for Serializer<'a> {
	type Ok = Value;
	type Error = SerializerError;
	type SerializeSeq = SerializeSequence<'a>;
	type SerializeTuple = SerializeSequence<'a>;
	type SerializeTupleStruct = SerializeSequence<'a>;
	type SerializeTupleVariant = SerializeTupleVariant<'a>;
	type SerializeMap = SerializeMap<'a>;
	type SerializeStruct = SerializeMap<'a>;
	type SerializeStructVariant = SerializeStructVariant<'a>;

	fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_bool(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_integer(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
		Err(SerializerError::Float)
	}

	fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
		Err(SerializerError::Float)
	}

	fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_char(self.context, value)?;
		self.expect_typed_value(value)
	}

	fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
		let value = match self.expected_type.kind() {
			TypeKind::String => Value::new_string(self.context, value)?,
			TypeKind::Array => {
				let element_type = TypeHandle::from_type_id(self.context.clone(), self.expected_type.type_parameters()[0]);
				match element_type.kind() {
					TypeKind::Char => {
						let chars: Result<Vec<Value>, _> = value.chars().map(|c| Value::new_char(self.context, c)).collect();
						Value::new_array(self.context, &element_type, chars?)?
					}
					_ => {
						return Err(SerializerError::InvalidTypeForString {
							expected: self.expected_type,
						})
					}
				}
			}
			_ => {
				return Err(SerializerError::InvalidTypeForString {
					expected: self.expected_type,
				})
			}
		};
		self.expect_typed_value(value)
	}

	fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
		let integer_type = self
			.context
			.get_integer_type()
			.map_err(crate::error::IntegerCreationError::InvalidType)?;
		let values = value
			.iter()
			.map(|byte| Value::new_integer(self.context, *byte))
			.collect::<Result<Vec<Value>, crate::error::IntegerCreationError>>()?;

		let value = Value::new_array(self.context, &integer_type, values)?;
		self.expect_typed_value(value)
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		let value = match self.expected_type.kind() {
			TypeKind::Enum => Value::new_enum_unit_variant(self.context, self.expected_type.name(), "None")?,
			_ => unimplemented!(),
		};
		self.expect_typed_value(value)
	}

	fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		let value = match self.expected_type.kind() {
			TypeKind::Enum => {
				let (variant_names, variants) = self.expected_type.variants();
				match variant_names.get("Some").map(|&id| &variants[id]) {
					Some(Variant::Tuple(_, element_types)) => {
						if element_types.len() != 1 {
							return Err(SerializerError::VariantMismatch {
								r#type: self.expected_type,
								variant_name: "Some".into(),
							});
						}
						Value::new_enum_tuple_variant(
							self.context,
							self.expected_type.name(),
							"Some",
							Box::new([Value::from_serde_by_type(
								self.context,
								value,
								TypeHandle::from_type_id(self.context.clone(), element_types[0]),
							)?]) as Box<[Value]>,
						)?
					}
					Some(_) => {
						return Err(SerializerError::VariantMismatch {
							r#type: self.expected_type,
							variant_name: "Some".into(),
						})
					}
					None => {
						return Err(SerializerError::InvalidVariant {
							expected: self.expected_type,
							variant_name: "Some".into(),
						})
					}
				}
			}
			_ => unimplemented!(),
		};
		self.expect_typed_value(value)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		let value = Value::new_unit(self.context)?;
		self.expect_typed_value(value)
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			let value = match self.expected_type.kind() {
				TypeKind::UnitStruct => Value::new_unit_struct(self.context, name)?,
				_ => unimplemented!(),
			};
			self.expect_typed_value(value)
		}
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			match self.expected_type.kind() {
				TypeKind::Enum => {
					let (variant_names, variants) = self.expected_type.variants();
					if let Some(variant_id) = variant_names.get(variant) {
						match variants[*variant_id] {
							Variant::Unit(_) => Ok(Value::new_enum_unit_variant(self.context, name, variant)?),
							Variant::Tuple(_, _) => Ok(Value::new_enum_tuple_variant(
								self.context,
								name,
								variant,
								vec![] as Vec<Value>,
							)?),
							Variant::Struct(_, _, _) => Ok(Value::new_enum_struct_variant(
								self.context,
								name,
								variant,
								(vec![] as Vec<(String, Value)>).into_iter(),
							)?),
						}
					} else {
						Err(SerializerError::InvalidVariant {
							expected: self.expected_type,
							variant_name: variant.into(),
						})
					}
				}
				_ => Err(SerializerError::InvalidTypeForVariant {
					expected: self.expected_type,
				}),
			}
		}
	}

	fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
		self,
		_name: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error> {
		unimplemented!("`serialize_newtype_struct` for type {}", self.expected_type.name())
	}

	fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
		self,
		name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			match self.expected_type.kind() {
				TypeKind::Enum => {
					let (variant_names, variants) = self.expected_type.variants();
					if let Some(variant_id) = variant_names.get(variant) {
						match &variants[*variant_id] {
							Variant::Tuple(_, types) => Ok(Value::new_enum_tuple_variant(
								self.context,
								name,
								variant,
								Box::new([Value::from_serde_by_type(
									self.context,
									value,
									TypeHandle::from_type_id(self.context.clone(), types[0]),
								)?]) as Box<[Value]>,
							)?),
							_ => Err(SerializerError::VariantMismatch {
								r#type: self.expected_type,
								variant_name: variant.into(),
							}),
						}
					} else {
						Err(SerializerError::InvalidVariant {
							expected: self.expected_type,
							variant_name: variant.into(),
						})
					}
				}
				_ => Err(SerializerError::InvalidTypeForVariant {
					expected: self.expected_type,
				}),
			}
		}
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		SerializeSequence::new(self.context, self.expected_type, len)
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		SerializeSequence::new(self.context, self.expected_type, len)
	}

	fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			SerializeSequence::new(self.context, self.expected_type, len)
		}
	}

	fn serialize_tuple_variant(
		self,
		name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			SerializeTupleVariant::new(self.context, self.expected_type, variant, len)
		}
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		SerializeMap::new(self.context, self.expected_type, len)
	}

	fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			SerializeMap::new(self.context, self.expected_type, len)
		}
	}

	fn serialize_struct_variant(
		self,
		name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		if (&*self.expected_type.name()) != name {
			Err(SerializerError::TypeNameMismatch {
				expected: self.expected_type,
				actual: name.into(),
			})
		} else {
			match self.expected_type.kind() {
				TypeKind::Enum => {
					let (variant_names, variants) = self.expected_type.variants();
					if let Some(&variant_id) = variant_names.get(variant) {
						match &variants[variant_id] {
							Variant::Struct(_, _, _) => {
								SerializeStructVariant::new(self.context, self.expected_type, variant_id, len)
							}
							_ => Err(SerializerError::VariantMismatch {
								r#type: self.expected_type,
								variant_name: variant.into(),
							}),
						}
					} else {
						Err(SerializerError::InvalidVariant {
							expected: self.expected_type,
							variant_name: variant.into(),
						})
					}
				}
				_ => Err(SerializerError::InvalidTypeForVariant {
					expected: self.expected_type,
				}),
			}
		}
	}
}
