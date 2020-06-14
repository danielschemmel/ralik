use num_bigint::BigInt;

use std::sync::Arc;

use crate::error::RuntimeError;
use crate::{Type, Value};

pub(crate) trait Arguments {
	fn check_len(&self, count: usize) -> Result<(), RuntimeError>;

	fn as_unit(&self, index: usize) -> Result<(), RuntimeError>;
	fn as_bool(&self, index: usize) -> Result<bool, RuntimeError>;
	fn as_char(&self, index: usize) -> Result<char, RuntimeError>;
	fn as_integer(&self, index: usize) -> Result<&BigInt, RuntimeError>;
	fn as_string(&self, index: usize) -> Result<&String, RuntimeError>;

	fn check_type(&self, index: usize, expected_type: &Arc<dyn Type>) -> Result<&Value, RuntimeError>;
}

impl Arguments for [Value] {
	fn check_len(&self, expected: usize) -> Result<(), RuntimeError> {
		if self.len() != expected {
			Err(RuntimeError::InvalidNumberOfArguments {
				actual: self.len(),
				expected,
			})
		} else {
			Ok(())
		}
	}

	fn as_unit(&self, index: usize) -> Result<(), RuntimeError> {
		self[index].as_unit().ok_or_else(|| RuntimeError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::unit_name().to_owned(),
		})
	}

	fn as_bool(&self, index: usize) -> Result<bool, RuntimeError> {
		self[index].as_bool().ok_or_else(|| RuntimeError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::bool_name().to_owned(),
		})
	}

	fn as_char(&self, index: usize) -> Result<char, RuntimeError> {
		self[index].as_char().ok_or_else(|| RuntimeError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::char_name().to_owned(),
		})
	}

	fn as_integer(&self, index: usize) -> Result<&BigInt, RuntimeError> {
		self[index]
			.as_integer()
			.ok_or_else(|| RuntimeError::InvalidArgumentType {
				argument_number: index,
				actual_type_name: self[index].get_type().name().to_string(),
				expected_type_name: super::integer_name().to_owned(),
			})
	}

	fn as_string(&self, index: usize) -> Result<&String, RuntimeError> {
		self[index]
			.as_string()
			.ok_or_else(|| RuntimeError::InvalidArgumentType {
				argument_number: index,
				actual_type_name: self[index].get_type().name().to_string(),
				expected_type_name: super::string_name().to_owned(),
			})
	}

	fn check_type(&self, index: usize, expected_type: &Arc<dyn Type>) -> Result<&Value, RuntimeError> {
		let value = &self[index];
		if value.has_type(expected_type) {
			Ok(value)
		} else {
			Err(RuntimeError::InvalidArgumentType {
				argument_number: index,
				actual_type_name: self[index].get_type().name().to_string(),
				expected_type_name: expected_type.name().to_owned(),
			})
		}
	}
}
