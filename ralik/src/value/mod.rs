use num_bigint::BigInt;

use std::sync::Arc;

use super::Type;

mod debug;

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
