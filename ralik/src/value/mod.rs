use num::BigInt;
use num::ToPrimitive;

use crate::types::Variant;
use crate::TypeHandle;

mod debug;
mod display;
mod new;

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

	pub fn as_variant_id(&self) -> Option<usize> {
		match &self.data {
			Data::UnitVariant(id) => Some(*id),
			Data::Variant(id, _) => Some(*id),
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
