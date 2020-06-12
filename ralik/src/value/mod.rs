use num_bigint::BigInt;
use num_traits::ToPrimitive;

use std::sync::Arc;

use super::Type;

mod debug;

#[cfg(feature = "serde")]
mod serializer;

#[derive(Clone)]
pub enum Value {
	Bool(bool),
	Char(char),
	Integer(BigInt),
	String(String),
}

impl Value {
	pub fn get_type(&self) -> Arc<Type> {
		match self {
			Value::Bool(_) => Type::bool(),
			Value::Char(_) => Type::char(),
			Value::Integer(_) => Type::integer(),
			Value::String(_) => Type::string(),
		}
	}

	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Value::Bool(value) => Some(*value),
			_ => None,
		}
	}

	pub fn is_bool(&self) -> bool {
		match self {
			Value::Bool(_value) => true,
			_ => false,
		}
	}

	pub fn as_char(&self) -> Option<char> {
		match self {
			Value::Char(value) => Some(*value),
			_ => None,
		}
	}

	pub fn is_char(&self) -> bool {
		match self {
			Value::Char(_value) => true,
			_ => false,
		}
	}

	pub fn as_integer(&self) -> Option<&BigInt> {
		match self {
			Value::Integer(value) => Some(value),
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

	pub fn is_integer(&self) -> bool {
		match self {
			Value::Integer(_value) => true,
			_ => false,
		}
	}

	pub fn as_string(&self) -> Option<&String> {
		match self {
			Value::String(value) => Some(value),
			_ => None,
		}
	}

	pub fn is_string(&self) -> bool {
		match self {
			Value::String(_value) => true,
			_ => false,
		}
	}

	pub fn field(&self, _name: &str) -> Option<&Value> {
		match self {
			Value::Bool(_) => None,
			Value::Char(_) => None,
			Value::Integer(_) => None,
			Value::String(_) => None,
		}
	}

	pub fn field_mut(&self, _name: &str) -> Option<&mut Value> {
		match self {
			Value::Bool(_) => None,
			Value::Char(_) => None,
			Value::Integer(_) => None,
			Value::String(_) => None,
		}
	}
}
