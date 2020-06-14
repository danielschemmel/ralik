use num_bigint::BigInt;

use crate::Value;

use super::CallError;

pub(crate) trait Arguments {
	fn check_len(&self, count: usize) -> Result<(), CallError>;
	
	fn as_unit(&self, index: usize) -> Result<(), CallError>;
	fn as_bool(&self, index: usize) -> Result<bool, CallError>;
	fn as_char(&self, index: usize) -> Result<char, CallError>;
	fn as_integer(&self, index: usize) -> Result<&BigInt, CallError>;
	fn as_string(&self, index: usize) -> Result<&String, CallError>;
}

impl Arguments for [Value] {
	fn check_len(&self, expected: usize) -> Result<(), CallError> {
		if self.len() != expected {
			Err(CallError::InvalidNumberOfArguments {
				actual: self.len(),
				expected,
			})
		} else {
			Ok(())
		}
	}

	fn as_unit(&self, index: usize) -> Result<(), CallError> {
		self[index].as_unit().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::UnitName.to_string(),
		})
	}

	fn as_bool(&self, index: usize) -> Result<bool, CallError> {
		self[index].as_bool().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::BoolName.to_string(),
		})
	}

	fn as_char(&self, index: usize) -> Result<char, CallError> {
		self[index].as_char().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::CharName.to_string(),
		})
	}

	fn as_integer(&self, index: usize) -> Result<&BigInt, CallError> {
		self[index].as_integer().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::IntegerName.to_string(),
		})
	}

	fn as_string(&self, index: usize) -> Result<&String, CallError> {
		self[index].as_string().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::StringName.to_string(),
		})
	}
}
