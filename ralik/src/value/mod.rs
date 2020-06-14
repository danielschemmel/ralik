use num_bigint::BigInt;
use num_traits::ToPrimitive;

use std::collections::hash_map::HashMap;

use crate::error::{
	ArrayCreationError, BoolCreationError, CharCreationError, IntegerCreationError, InvalidArrayType,
	StringCreationError, StructCreationError, TupleCreationError, UnitCreationError,
};
use crate::types::TypeKind;
use crate::{Context, TypeHandle};

mod debug;

#[cfg(feature = "serde")]
mod serializer;

#[derive(Clone)]
pub struct Value {
	r#type: TypeHandle,
	data: Data,
}

#[derive(Clone)]
enum Data {
	Unit,
	Bool(bool),
	Integer(BigInt),
	Char(char),
	String(String),
	Tuple(Vec<Value>),
	Struct(HashMap<String, Value>),
	Array(Vec<Value>),
}

impl Value {
	pub fn new_unit(context: &Context) -> Result<Value, UnitCreationError> {
		Ok(Value {
			r#type: context.get_unit_type()?.clone(),
			data: Data::Unit,
		})
	}

	pub fn new_bool(context: &Context, value: bool) -> Result<Value, BoolCreationError> {
		Ok(Value {
			r#type: context.get_bool_type()?.clone(),
			data: Data::Bool(value),
		})
	}

	pub fn new_char(context: &Context, value: char) -> Result<Value, CharCreationError> {
		Ok(Value {
			r#type: context.get_char_type()?.clone(),
			data: Data::Char(value),
		})
	}

	pub fn new_integer(context: &Context, value: impl Into<BigInt>) -> Result<Value, IntegerCreationError> {
		Ok(Value {
			r#type: context.get_integer_type()?.clone(),
			data: Data::Integer(value.into()),
		})
	}

	pub fn new_string(context: &Context, value: impl Into<String>) -> Result<Value, StringCreationError> {
		Ok(Value {
			r#type: context.get_string_type()?.clone(),
			data: Data::String(value.into()),
		})
	}

	pub fn new_tuple(context: &Context, values: impl Into<Vec<Value>>) -> Result<Value, TupleCreationError> {
		let values: Vec<Value> = values.into();
		let element_types = values
			.iter()
			.map(|value| value.get_type().name())
			.collect::<Vec<&str>>();
		let tuple_type = context.get_tuple_type(&element_types)?.clone();
		Ok(Value {
			r#type: tuple_type,
			data: Data::Tuple(values),
		})
	}

	pub fn new_struct(
		context: &Context,
		name: &str,
		fields: impl Iterator<Item = (impl Into<String>, Value)>,
	) -> Result<Value, StructCreationError> {
		let struct_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidStructType::Missing {
				type_name: name.to_string(),
			})?
			.clone();
		if struct_type.kind() != TypeKind::Struct {
			return Err(
				crate::error::InvalidStructType::NotStructType {
					type_name: name.to_string(),
				}
				.into(),
			);
		}

		let fields: HashMap<String, Value> = fields.map(|(name, value)| (name.into(), value)).collect();
		let field_map = struct_type.fields().unwrap();
		for (field_name, field_type) in field_map {
			let value = fields
				.get(field_name)
				.ok_or_else(|| StructCreationError::MissingField {
					type_name: name.to_string(),
					field_name: field_name.to_string(),
				})?;
			let value_type = value.get_type();
			if !TypeHandle::is_same(value_type, field_type) {
				return Err(StructCreationError::FieldTypeMismatch {
					type_name: name.to_string(),
					field_name: field_name.to_string(),
					field_type_name: field_type.name().to_string(),
					value_type_name: value_type.name().to_string(),
				});
			}
		}

		// After establishing that `field_map.keys()` \subseteq `fields.keys()`, we can shortcut the reverse check if the
		// lengths are equal, as this implies that `field_map.keys()` = `fields.keys()`
		if fields.len() != field_map.len() {
			for field_name in fields.keys() {
				field_map
					.get(field_name)
					.ok_or_else(|| StructCreationError::SuperfluousField {
						type_name: name.to_string(),
						field_name: field_name.to_string(),
					})?;
			}
		}

		Ok(Value {
			r#type: struct_type,
			data: Data::Struct(fields),
		})
	}

	pub fn new_unit_struct(context: &Context, name: &str) -> Result<Value, StructCreationError> {
		let struct_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidStructType::Missing {
				type_name: name.to_string(),
			})?
			.clone();
		if struct_type.kind() != TypeKind::Struct {
			return Err(
				crate::error::InvalidStructType::NotStructType {
					type_name: name.to_string(),
				}
				.into(),
			);
		}

		let field_map = struct_type.fields().unwrap();
		if field_map.is_empty() {
			Ok(Value {
				r#type: struct_type,
				data: Data::Struct(HashMap::new()),
			})
		} else {
			Err(StructCreationError::MissingField {
				type_name: name.to_string(),
				field_name: field_map.keys().nth(0).unwrap().to_string(),
			})
		}
	}

	pub fn new_array(
		context: &Context,
		element_type: &TypeHandle,
		values: impl Into<Vec<Value>>,
	) -> Result<Value, ArrayCreationError> {
		let values: Vec<Value> = values.into();
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

		let array_type = context.get_array_type(element_type.name())?.clone();
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

	pub fn is_unit(&self) -> bool {
		match &self.data {
			Data::Unit => true,
			_ => false,
		}
	}

	pub fn as_unit(&self) -> Option<()> {
		match &self.data {
			Data::Unit => Some(()),
			_ => None,
		}
	}

	pub fn is_bool(&self) -> bool {
		match &self.data {
			Data::Bool(_value) => true,
			_ => false,
		}
	}

	pub fn as_bool(&self) -> Option<bool> {
		match &self.data {
			Data::Bool(value) => Some(*value),
			_ => None,
		}
	}

	pub fn is_char(&self) -> bool {
		match &self.data {
			Data::Char(_value) => true,
			_ => false,
		}
	}

	pub fn as_char(&self) -> Option<char> {
		match &self.data {
			Data::Char(value) => Some(*value),
			_ => None,
		}
	}

	pub fn is_integer(&self) -> bool {
		match &self.data {
			Data::Integer(_value) => true,
			_ => false,
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

	pub fn is_string(&self) -> bool {
		match &self.data {
			Data::String(_value) => true,
			_ => false,
		}
	}

	pub fn as_string(&self) -> Option<&String> {
		match &self.data {
			Data::String(value) => Some(value),
			_ => None,
		}
	}

	pub fn is_tuple(&self) -> bool {
		match &self.data {
			Data::Tuple(_value) => true,
			_ => false,
		}
	}

	pub fn as_tuple(&self) -> Option<&[Value]> {
		match &self.data {
			Data::Tuple(value) => Some(value),
			_ => None,
		}
	}

	pub fn is_array(&self) -> bool {
		match &self.data {
			Data::Array(_value) => true,
			_ => false,
		}
	}

	pub fn as_array(&self) -> Option<&[Value]> {
		match &self.data {
			Data::Array(value) => Some(value),
			_ => None,
		}
	}

	pub fn field(&self, name: &str) -> Option<&Value> {
		match &self.data {
			Data::Struct(fields) => fields.get(name),
			_ => None,
		}
	}

	pub fn tuple_field(&self, index: usize) -> Option<&Value> {
		self.as_tuple().and_then(|slice| slice.get(index))
	}
}
