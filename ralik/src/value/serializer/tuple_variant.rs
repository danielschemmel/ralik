use my_serde::ser;

use super::SerializerError;

use crate::types::{TypeKind, Variant};
use crate::{Context, TypeHandle, Value};

pub(super) struct SerializeTupleVariant<'a> {
	context: &'a Context,
	expected_type: TypeHandle,
	variant_name: &'a str,
	element_types: Vec<TypeHandle>,
	result: Vec<Value>,
}

impl<'a> SerializeTupleVariant<'a> {
	pub fn new(
		context: &'a Context,
		expected_type: TypeHandle,
		variant_name: &'a str,
		len: usize,
	) -> Result<Self, SerializerError> {
		match expected_type.kind() {
			TypeKind::Enum => {
				let (variant_names, variants) = expected_type.variants().unwrap();
				match variant_names.get(variant_name).map(|id| &variants[*id]) {
					Some(Variant::Tuple(_, types)) => {
						let element_types = types.iter().rev().cloned().collect();
						Ok(Self {
							context,
							expected_type,
							variant_name,
							element_types,
							result: Vec::with_capacity(len),
						})
					}
					Some(Variant::Unit(variant_name)) | Some(Variant::Struct(variant_name, _, _)) => {
						let variant_name = variant_name.clone().into_string();
						Err(SerializerError::VariantMismatch {
							r#type: expected_type,
							variant_name,
						})
					}
					None => Err(SerializerError::InvalidVariant {
						expected: expected_type,
						variant_name: variant_name.into(),
					}),
				}
			}
			_ => {
				return Err(SerializerError::InvalidTypeForVariant {
					expected: expected_type,
				})
			}
		}
	}
}

impl<'a> SerializeTupleVariant<'a> {
	fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), SerializerError> {
		self.result.push(Value::from_serde_by_type(
			self.context,
			value,
			self.element_types.pop().unwrap(),
		)?);

		Ok(())
	}

	fn end(self) -> Result<Value, SerializerError> {
		if !self.element_types.is_empty() {
			return Err(SerializerError::TooFewValues {
				r#type: self.expected_type,
				count: self.element_types.len(),
			});
		}

		Ok(Value::new_enum_tuple_variant(
			self.context,
			self.expected_type.name(),
			self.variant_name,
			self.result,
		)?)
	}
}

impl<'a> ser::SerializeTupleVariant for SerializeTupleVariant<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_field<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		Self::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}
