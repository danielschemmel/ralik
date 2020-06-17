use my_serde::ser;

use super::SerializerError;

use crate::types::TypeKind;
use crate::{Context, TypeHandle, Value};

enum ElementTypes {
	Repeating(TypeHandle),
	Consuming(Vec<TypeHandle>),
}

pub(super) struct SerializeSequence<'a> {
	context: &'a Context,
	expected_type: TypeHandle,
	element_types: ElementTypes,
	result: Vec<Value>,
}

impl<'a> SerializeSequence<'a> {
	pub fn new(
		context: &'a Context,
		expected_type: TypeHandle,
		len: impl Into<Option<usize>>,
	) -> Result<Self, SerializerError> {
		let element_types = match expected_type.kind() {
			TypeKind::Tuple | TypeKind::TupleStruct => {
				ElementTypes::Consuming(expected_type.fields().1.iter().cloned().rev().collect())
			}
			TypeKind::Array => ElementTypes::Repeating(expected_type.type_parameters()[0].clone()),
			_ => {
				return Err(SerializerError::InvalidTypeForSequence {
					expected: expected_type,
				})
			}
		};

		Ok(Self {
			context,
			expected_type,
			element_types,
			result: Vec::with_capacity(len.into().unwrap_or(0)),
		})
	}
}

impl<'a> SerializeSequence<'a> {
	fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), SerializerError> {
		match &mut self.element_types {
			ElementTypes::Repeating(element_type) => {
				self
					.result
					.push(Value::from_serde_by_type(self.context, value, element_type.clone())?);
				Ok(())
			}
			ElementTypes::Consuming(element_type_stack) => {
				self.result.push(Value::from_serde_by_type(
					self.context,
					value,
					element_type_stack.pop().unwrap(),
				)?);
				Ok(())
			}
		}
	}

	fn end(self) -> Result<Value, SerializerError> {
		match self.element_types {
			ElementTypes::Consuming(element_type_stack) => {
				if !element_type_stack.is_empty() {
					return Err(SerializerError::TooFewValues {
						r#type: self.expected_type,
						count: element_type_stack.len(),
					});
				}
			}
			_ => (),
		}

		match self.expected_type.kind() {
			TypeKind::Tuple => {
				let value = Value::new_tuple(self.context, self.result)?;
				if value.has_type(&self.expected_type) {
					Ok(value)
				} else {
					Err(SerializerError::TypeMismatch {
						expected: self.expected_type,
						actual: value.get_type().clone(),
					})
				}
			}
			TypeKind::TupleStruct => {
				let value = Value::new_tuple_struct(self.context, self.expected_type.name(), self.result)?;
				if value.has_type(&self.expected_type) {
					Ok(value)
				} else {
					Err(SerializerError::TypeMismatch {
						expected: self.expected_type,
						actual: value.get_type().clone(),
					})
				}
			}
			TypeKind::Array => {
				let value = Value::new_array(self.context, &self.expected_type.type_parameters()[0], self.result)?;
				assert!(value.has_type(&self.expected_type));
				Ok(value)
			}
			_ => unreachable!(),
		}
	}
}

impl<'a> ser::SerializeSeq for SerializeSequence<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_element<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		Self::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}

impl<'a> ser::SerializeTuple for SerializeSequence<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_element<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		Self::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}

impl<'a> ser::SerializeTupleStruct for SerializeSequence<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_field<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		Self::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}
