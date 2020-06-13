use num_bigint::BigInt;
use num_traits::ToPrimitive;

use std::sync::Arc;

use crate::{
	Context, InvalidBoolType, InvalidCharType, InvalidIntegerType, InvalidStringType, InvalidVecGeneric, Type,
};

mod debug;

#[cfg(feature = "serde")]
mod serializer;

#[derive(Clone)]
pub struct Value {
	r#type: Arc<dyn Type>,
	data: Data,
}

#[derive(Clone)]
enum Data {
	Bool(bool),
	Char(char),
	Integer(BigInt),
	String(String),
	Vec(Vec<Value>),
}

impl Value {
	pub fn new_bool(context: &Context, value: bool) -> Result<Value, InvalidBoolType> {
		Ok(Value {
			r#type: context.get_bool_type()?.clone(),
			data: Data::Bool(value),
		})
	}

	pub fn new_char(context: &Context, value: char) -> Result<Value, InvalidCharType> {
		Ok(Value {
			r#type: context.get_char_type()?.clone(),
			data: Data::Char(value),
		})
	}

	pub fn new_integer(context: &Context, value: impl Into<BigInt>) -> Result<Value, InvalidIntegerType> {
		Ok(Value {
			r#type: context.get_integer_type()?.clone(),
			data: Data::Integer(value.into()),
		})
	}

	pub fn new_string(context: &Context, value: impl Into<String>) -> Result<Value, InvalidStringType> {
		Ok(Value {
			r#type: context.get_string_type()?.clone(),
			data: Data::String(value.into()),
		})
	}

	pub fn new_vec(context: &mut Context, value: Vec<Value>) -> Result<Value, InvalidVecGeneric> {
		unimplemented!()
	}
}

impl Value {
	pub fn get_type(&self) -> &Arc<dyn Type> {
		&self.r#type
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
	/*
	pub fn is_array(&self) -> bool {
		match self {
			Value::Array(_value) => true,
			_ => false,
		}
	}

	pub fn as_array(&self) -> Option<&Vec<Value>> {
		match self {
			Value::Array(value) => Some(value),
			_ => None,
		}
	}

	pub fn is_option(&self) -> bool {
		match self {
			Value::Option(_value) => true,
			_ => false,
		}
	}

	pub fn as_option(&self) -> Option<Option<&Value>> {
		match self {
			Value::Option(value) => Some(value.as_ref().map(|boxed| boxed.as_ref())),
			_ => None,
		}
	}*/

	pub fn field(&self, _name: &str) -> Option<&Value> {
		None
	}

	pub fn field_mut(&self, _name: &str) -> Option<&mut Value> {
		None
	}
}
