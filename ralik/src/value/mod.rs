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

	pub fn field(&self, name: &str) -> Option<&Value> {
		None
	}

	pub fn field_mut(&self, name: &str) -> Option<&mut Value> {
		None
	}
}
