use num::BigInt;

use crate::error::RuntimeError;
use crate::{Context, TypeHandle, Value};

pub(crate) trait Arguments {
	fn check_len(&self, count: usize) -> Result<(), RuntimeError>;

	fn as_bool(&self, index: usize, context: &Context) -> Result<bool, RuntimeError>;
	fn as_char(&self, index: usize, context: &Context) -> Result<char, RuntimeError>;
	fn as_integer(&self, index: usize, context: &Context) -> Result<&BigInt, RuntimeError>;
	fn as_string(&self, index: usize, context: &Context) -> Result<&str, RuntimeError>;

	fn check_type(&self, index: usize, expected_type: &TypeHandle) -> Result<&Value, RuntimeError>;
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

	fn as_bool(&self, index: usize, context: &Context) -> Result<bool, RuntimeError> {
		Ok(
			self
				.check_type(
					index,
					&context
						.get_bool_type()
						.map_err(|err| RuntimeError::InvalidCoreType(err.into()))?,
				)?
				.as_bool()
				.unwrap(),
		)
	}

	fn as_char(&self, index: usize, context: &Context) -> Result<char, RuntimeError> {
		Ok(
			self
				.check_type(
					index,
					&context
						.get_char_type()
						.map_err(|err| RuntimeError::InvalidCoreType(err.into()))?,
				)?
				.as_char()
				.unwrap(),
		)
	}

	fn as_integer(&self, index: usize, context: &Context) -> Result<&BigInt, RuntimeError> {
		Ok(
			self
				.check_type(
					index,
					&context
						.get_integer_type()
						.map_err(|err| RuntimeError::InvalidCoreType(err.into()))?,
				)?
				.as_integer()
				.unwrap(),
		)
	}

	fn as_string(&self, index: usize, context: &Context) -> Result<&str, RuntimeError> {
		Ok(
			self
				.check_type(
					index,
					&context
						.get_string_type()
						.map_err(|err| RuntimeError::InvalidCoreType(err.into()))?,
				)?
				.as_string()
				.unwrap(),
		)
	}

	fn check_type(&self, index: usize, expected_type: &TypeHandle) -> Result<&Value, RuntimeError> {
		let value = &self[index];
		if value.has_type(expected_type) {
			Ok(value)
		} else {
			Err(RuntimeError::InvalidArgumentType {
				argument_number: index,
				actual_type_name: (&*self[index].get_type().name()).into(),
				expected_type_name: (&*expected_type.name()).to_owned(),
			})
		}
	}
}
