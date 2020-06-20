use my_serde::ser;

use std::collections::hash_map::{Entry, HashMap};

use super::SerializerError;

use crate::types::{TypeKind, Variant};
use crate::{Context, TypeHandle, Value};

pub(super) struct SerializeStructVariant<'a> {
	context: &'a Context,
	expected_type: TypeHandle,
	variant_id: usize,
	result: HashMap<String, Value>,
}

impl<'a> SerializeStructVariant<'a> {
	pub fn new(
		context: &'a Context,
		expected_type: TypeHandle,
		variant_id: usize,
		len: usize,
	) -> Result<Self, SerializerError> {
		match expected_type.kind() {
			TypeKind::Enum => match &expected_type.variants().1[variant_id] {
				Variant::Struct(_, _, _) => Ok(Self {
					context,
					expected_type,
					variant_id,
					result: HashMap::with_capacity(len),
				}),
				Variant::Unit(variant_name) | Variant::Tuple(variant_name, _) => {
					let variant_name = variant_name.clone().into_string();
					Err(SerializerError::VariantMismatch {
						r#type: expected_type,
						variant_name,
					})
				}
			},
			_ => {
				return Err(SerializerError::InvalidTypeForVariant {
					expected: expected_type,
				})
			}
		}
	}

	fn serialize_field<T: ser::Serialize + ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), SerializerError> {
		match &self.expected_type.variants().1[self.variant_id] {
			Variant::Struct(_, field_names, field_types) => {
				let value = if let Some(key_type) = field_names.get(key).map(|id| &field_types[*id]) {
					Value::from_serde_by_type(
						self.context,
						value,
						TypeHandle::from_type_id(self.context.clone(), *key_type),
					)?
				} else {
					return Err(SerializerError::UnexpectedKey {
						r#type: self.expected_type.clone(),
						key: key.into(),
					});
				};

				match self.result.entry(key.into()) {
					Entry::Occupied(_) => Err(SerializerError::DuplicateKey {
						r#type: self.expected_type.clone(),
						key: key.into(),
					}),
					Entry::Vacant(entry) => {
						entry.insert(value);
						Ok(())
					}
				}
			}
			_ => Err(SerializerError::UnexpectedKey {
				r#type: self.expected_type.clone(),
				key: key.into(),
			}),
		}
	}

	fn end(self) -> Result<Value, SerializerError> {
		let value = Value::new_enum_struct_variant(
			self.context,
			self.expected_type.name(),
			self.expected_type.variants().1[self.variant_id].name(),
			self.result.into_iter(),
		)?;
		assert!(value.has_type(&self.expected_type));
		Ok(value)
	}
}

impl<'a> ser::SerializeStruct for SerializeStructVariant<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_field<T: ser::Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		Self::serialize_field(self, key, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}

impl<'a> ser::SerializeStructVariant for SerializeStructVariant<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_field<T: ser::Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		Self::serialize_field(self, key, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}
