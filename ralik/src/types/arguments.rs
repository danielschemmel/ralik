use num_bigint::BigInt;

use crate::Value;

use super::CallError;

pub(crate) trait Arguments {
	fn check_len(&self, count: usize) -> Result<(), CallError>;
	fn as_bool(&self, index: usize) -> Result<bool, CallError>;
	fn as_integer(&self, index: usize) -> Result<&BigInt, CallError>;
}

impl Arguments for [Value] {
	fn as_bool(&self, index: usize) -> Result<bool, CallError> {
		self[index].as_bool().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::bool::NAME.to_string(),
		})
	}

	fn as_integer(&self, index: usize) -> Result<&BigInt, CallError> {
		self[index].as_integer().ok_or_else(|| CallError::InvalidArgumentType {
			argument_number: index,
			actual_type_name: self[index].get_type().name().to_string(),
			expected_type_name: super::integer::NAME.to_string(),
		})
	}

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
}
