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
