use num::BigInt;
use num::ToPrimitive;

use crate::error::{
	ArrayCreationError, BoolCreationError, CharCreationError, IntegerCreationError, InvalidArrayType,
	StringCreationError, StructCreationError, TupleCreationError,
};
use crate::types::{TypeKind, Variant};
use crate::{Context, TypeHandle};

mod debug;
mod display;

#[cfg(feature = "serde")]
mod serializer;

#[derive(Clone, Debug)]
pub struct Value {
	r#type: TypeHandle,
	data: Data,
}

#[derive(Clone)]
enum Data {
	Empty,
	Bool(bool),
	Integer(BigInt),
	Char(char),
	String(Box<str>),
	Array(Box<[Value]>),
	UnitVariant(usize),
	Variant(usize, Box<[Value]>),
}

impl Value {
	pub fn new_unit(context: &Context) -> Result<Value, TupleCreationError> {
		Ok(Value {
			r#type: context.get_unit_type()?.clone(),
			data: Data::Empty,
		})
	}

	pub fn new_bool(context: &Context, value: impl Into<bool>) -> Result<Value, BoolCreationError> {
		Ok(Value {
			r#type: context.get_bool_type()?.clone(),
			data: Data::Bool(value.into()),
		})
	}

	pub fn new_char(context: &Context, value: impl Into<char>) -> Result<Value, CharCreationError> {
		Ok(Value {
			r#type: context.get_char_type()?.clone(),
			data: Data::Char(value.into()),
		})
	}

	pub fn new_integer(context: &Context, value: impl Into<BigInt>) -> Result<Value, IntegerCreationError> {
		Ok(Value {
			r#type: context.get_integer_type()?.clone(),
			data: Data::Integer(value.into()),
		})
	}

	pub fn new_string(context: &Context, value: impl Into<Box<str>>) -> Result<Value, StringCreationError> {
		Ok(Value {
			r#type: context.get_string_type()?.clone(),
			data: Data::String(value.into()),
		})
	}

	pub fn new_tuple(context: &Context, values: impl Into<Box<[Value]>>) -> Result<Value, TupleCreationError> {
		let values: Box<[Value]> = values.into();
		let element_types = values.iter().map(|value| value.get_type().name());
		let tuple_type = context.get_tuple_type(element_types)?.clone();
		Ok(Value {
			r#type: tuple_type,
			data: Data::Array(values),
		})
	}

	pub fn new_tuple_struct(
		context: &Context,
		name: impl AsRef<str>,
		values: impl Into<Box<[Value]>>,
	) -> Result<Value, TupleCreationError> {
		let tuple_type = context
			.get_type(name.as_ref())
			.ok_or_else(|| crate::error::InvalidTupleType::Missing {
				type_name: name.as_ref().into(),
			})?
			.clone();
		if tuple_type.kind() != TypeKind::Tuple {
			return Err(
				crate::error::InvalidTupleType::NotTupleType {
					type_name: name.as_ref().into(),
				}
				.into(),
			);
		}

		let values: Box<[Value]> = values.into();
		let element_types = tuple_type.fields().1;
		if values.len() != element_types.len() {
			return Err(crate::error::TupleCreationError::ElementCount {
				type_element_count: element_types.len(),
				provided_element_count: values.len(),
			});
		}

		if values.is_empty() {
			Ok(Value {
				r#type: tuple_type,
				data: Data::Empty,
			})
		} else {
			for (index, (value, expected_type)) in values.iter().zip(element_types.iter()).enumerate() {
				let value_type = value.get_type();
				if !TypeHandle::is_same(value_type, expected_type) {
					return Err(crate::error::TupleCreationError::ElementTypeMismatch {
						index,
						expected: expected_type.clone(),
						actual: value_type.clone(),
					});
				}
			}

			Ok(Value {
				r#type: tuple_type,
				data: Data::Array(values),
			})
		}
	}

	pub fn new_struct(
		context: &Context,
		name: impl AsRef<str>,
		mut fields: impl Iterator<Item = (impl AsRef<str>, Value)>,
	) -> Result<Value, StructCreationError> {
		let name = name.as_ref();

		let struct_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidStructType::Missing { type_name: name.into() })?
			.clone();
		if struct_type.kind() != TypeKind::Struct {
			return Err(crate::error::InvalidStructType::NotStructType { r#type: struct_type }.into());
		}

		let (field_names, field_types) = struct_type.fields();

		if field_types.is_empty() {
			if let Some((field_name, _field_value)) = fields.nth(0) {
				Err(StructCreationError::SuperfluousField {
					r#type: struct_type,
					field_name: field_name.as_ref().into(),
				})
			} else {
				Ok(Value {
					r#type: struct_type,
					data: Data::Empty,
				})
			}
		} else {
			if field_names.is_none() {
				return Err(crate::error::InvalidStructType::NoFieldNames { r#type: struct_type }.into());
			}
			let field_names = field_names.unwrap();

			let mut fields = fields
				.map(|(field_name, value)| {
					if let Some(index) = field_names.get(field_name.as_ref()) {
						Ok((*index, field_name, value))
					} else {
						Err(StructCreationError::SuperfluousField {
							r#type: struct_type.clone(),
							field_name: field_name.as_ref().into(),
						})
					}
				})
				.collect::<Result<Vec<(usize, _, Value)>, StructCreationError>>()?;

			fields.sort_unstable_by_key(|(key, _name, _value)| *key);

			let fields = fields
				.into_iter()
				.enumerate()
				.map(|(i, (key, name, value))| {
					if i < key {
						Err(StructCreationError::MissingField {
							r#type: struct_type.clone(),
							field_name: name.as_ref().into(),
						})
					} else if i > key {
						Err(StructCreationError::DuplicateField {
							r#type: struct_type.clone(),
							field_name: name.as_ref().into(),
						})
					} else if !value.has_type(&field_types[key]) {
						Err(StructCreationError::FieldTypeMismatch {
							r#type: struct_type.clone(),
							field_name: name.as_ref().into(),
							field_type: field_types[key].clone(),
							value_type: value.get_type().clone(),
						})
					} else {
						Ok(value)
					}
				})
				.collect::<Result<Vec<Value>, StructCreationError>>()?;

			Ok(Value {
				r#type: struct_type,
				data: Data::Array(fields.into_boxed_slice()),
			})
		}
	}

	pub fn new_unit_struct(context: &Context, name: impl AsRef<str>) -> Result<Value, StructCreationError> {
		let name = name.as_ref();

		let struct_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidStructType::Missing { type_name: name.into() })?
			.clone();
		if struct_type.kind() != TypeKind::Struct {
			return Err(crate::error::InvalidStructType::NotStructType { r#type: struct_type }.into());
		}

		let (field_names, field_types) = struct_type.fields();
		if field_types.is_empty() {
			Ok(Value {
				r#type: struct_type,
				data: Data::Empty,
			})
		} else {
			if let Some(field_names) = field_names {
				let field_name = field_names.keys().nth(0).unwrap().clone().into_string();
				Err(StructCreationError::MissingField {
					r#type: struct_type,
					field_name,
				})
			} else {
				Err(crate::error::InvalidStructType::NoFieldNames { r#type: struct_type }.into())
			}
		}
	}

	pub fn new_array(
		context: &Context,
		element_type: &TypeHandle,
		values: impl Into<Box<[Value]>>,
	) -> Result<Value, ArrayCreationError> {
		let values: Box<[Value]> = values.into();

		if let Some((index, value)) = values
			.iter()
			.enumerate()
			.find(|(_index, value)| !value.has_type(element_type))
		{
			return Err(
				InvalidArrayType::InvalidElement {
					value: value.clone(),
					index,
					type_name: crate::types::array_name(element_type.name()),
				}
				.into(),
			);
		}

		let array_type = context.get_array_type(element_type.name())?.into();
		Ok(Value {
			r#type: array_type,
			data: Data::Array(values),
		})
	}
}

impl Value {
	pub fn get_type(&self) -> &TypeHandle {
		&self.r#type
	}

	pub fn has_type(&self, expected_type: &TypeHandle) -> bool {
		TypeHandle::is_same(&self.r#type, expected_type)
	}

	pub fn as_nothing(&self) -> Option<()> {
		match &self.data {
			Data::Empty => Some(()),
			Data::UnitVariant(_id) => Some(()),
			_ => None,
		}
	}

	pub fn as_bool(&self) -> Option<bool> {
		match &self.data {
			Data::Bool(value) => Some(*value),
			_ => None,
		}
	}

	pub fn as_char(&self) -> Option<char> {
		match &self.data {
			Data::Char(value) => Some(*value),
			_ => None,
		}
	}

	pub fn as_integer(&self) -> Option<&BigInt> {
		match &self.data {
			Data::Integer(value) => Some(value),
			_ => None,
		}
	}

	pub fn as_i8(&self) -> Option<i8> {
		self.as_integer().and_then(|value| value.to_i8())
	}

	pub fn as_u8(&self) -> Option<u8> {
		self.as_integer().and_then(|value| value.to_u8())
	}

	pub fn as_i16(&self) -> Option<i16> {
		self.as_integer().and_then(|value| value.to_i16())
	}

	pub fn as_u16(&self) -> Option<u16> {
		self.as_integer().and_then(|value| value.to_u16())
	}

	pub fn as_i32(&self) -> Option<i32> {
		self.as_integer().and_then(|value| value.to_i32())
	}

	pub fn as_u32(&self) -> Option<u32> {
		self.as_integer().and_then(|value| value.to_u32())
	}

	pub fn as_i64(&self) -> Option<i64> {
		self.as_integer().and_then(|value| value.to_i64())
	}

	pub fn as_u64(&self) -> Option<u64> {
		self.as_integer().and_then(|value| value.to_u64())
	}

	pub fn as_i128(&self) -> Option<i128> {
		self.as_integer().and_then(|value| value.to_i128())
	}

	pub fn as_u128(&self) -> Option<u128> {
		self.as_integer().and_then(|value| value.to_u128())
	}

	pub fn as_isize(&self) -> Option<isize> {
		self.as_integer().and_then(|value| value.to_isize())
	}

	pub fn as_usize(&self) -> Option<usize> {
		self.as_integer().and_then(|value| value.to_usize())
	}

	pub fn as_array(&self) -> Option<&[Value]> {
		match &self.data {
			Data::Array(value) => Some(value.as_ref()),
			Data::Variant(_id, value) => Some(value.as_ref()),
			_ => None,
		}
	}

	pub fn as_string(&self) -> Option<&str> {
		match &self.data {
			Data::String(value) => Some(value.as_ref()),
			_ => None,
		}
	}

	pub fn field(&self, name: &str) -> Option<&Value> {
		match &self.data {
			Data::Array(fields) => {
				let field_names = self.r#type.fields().0?;
				Some(&fields[*field_names.get(name)?])
			}
			Data::Variant(id, fields) => {
				let variant = &self.r#type.variants()?.1[*id];
				match variant {
					Variant::Struct(_name, field_names, _field_types) => Some(&fields[*field_names.get(name)?]),
					_ => None,
				}
			}
			_ => None,
		}
	}

	pub fn tuple_field(&self, index: usize) -> Option<&Value> {
		match &self.data {
			Data::Array(elements) => elements.get(index),
			Data::Variant(_id, elements) => elements.get(index),
			_ => None,
		}
	}
}
