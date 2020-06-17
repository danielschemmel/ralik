use my_serde::ser;

use std::collections::hash_map::{Entry, HashMap};

use super::SerializerError;

use crate::types::TypeKind;
use crate::{Context, TypeHandle, Value};

enum Variant {
	Struct {
		last_key: Option<String>,
		string_type: TypeHandle, // FIXME: there are better was to serialize to a string
		result: HashMap<String, Value>,
	},
	#[allow(dead_code)]
	Value {
		last_key: Option<Value>,
		result: HashMap<Value, Value>,
	},
}

pub(super) struct SerializeMap<'a> {
	context: &'a Context,
	expected_type: TypeHandle,
	variant: Variant,
}

impl<'a> SerializeMap<'a> {
	pub fn new(
		context: &'a Context,
		expected_type: TypeHandle,
		len: impl Into<Option<usize>>,
	) -> Result<Self, SerializerError> {
		let key_type = match expected_type.kind() {
			TypeKind::Struct => Variant::Struct {
				last_key: None,
				string_type: context.get_string_type()?,
				result: HashMap::with_capacity(len.into().unwrap_or(0)),
			},
			_ => {
				return Err(SerializerError::InvalidTypeForMap {
					expected: expected_type,
				})
			}
		};

		Ok(Self {
			context,
			expected_type,
			variant: key_type,
		})
	}

	fn serialize_key<T: ser::Serialize + ?Sized>(&mut self, key: &T) -> Result<(), SerializerError> {
		match &mut self.variant {
			Variant::Struct {
				last_key,
				string_type,
				result: _,
			} => {
				assert!(last_key.is_none());
				*last_key = Some(
					Value::from_serde_by_type(self.context, key, string_type.clone())?
						.as_string()
						.unwrap()
						.to_owned(),
				);

				Ok(())
			}
			_ => unreachable!(),
		}
	}

	fn serialize_value<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), SerializerError> {
		match &mut self.variant {
			Variant::Struct {
				last_key,
				string_type: _,
				result,
			} => {
				assert!(last_key.is_some());
				let mut key = None;
				std::mem::swap(&mut key, last_key);
				let key = key.unwrap();

				let value = match self.expected_type.kind() {
					TypeKind::Struct => {
						let (field_names, field_types) = self.expected_type.fields();
						if let Some(key_type) = field_names
							.and_then(|field_names| field_names.get(&key as &str))
							.map(|id| &field_types[*id])
						{
							Value::from_serde_by_type(self.context, value, key_type.clone())?
						} else {
							return Err(SerializerError::UnexpectedKey {
								r#type: self.expected_type.clone(),
								key,
							});
						}
					}
					_ => unreachable!(),
				};

				match result.entry(key) {
					Entry::Occupied(entry) => Err(SerializerError::DuplicateKey {
						r#type: self.expected_type.clone(),
						key: entry.key().clone(),
					}),
					Entry::Vacant(entry) => {
						entry.insert(value);
						Ok(())
					}
				}
			}
			_ => unreachable!(),
		}
	}

	fn serialize_entry<K: ser::Serialize + ?Sized, V: ser::Serialize + ?Sized>(
		&mut self,
		key: &K,
		value: &V,
	) -> Result<(), SerializerError> {
		match &mut self.variant {
			Variant::Struct {
				last_key,
				string_type,
				result,
			} => {
				assert!(last_key.is_none());
				let key_value = Value::from_serde_by_type(self.context, key, string_type.clone())?;
				let key = key_value.as_string().unwrap();

				let value = match self.expected_type.kind() {
					TypeKind::Struct => {
						let (field_names, field_types) = self.expected_type.fields();
						if let Some(key_type) = field_names
							.and_then(|field_names| field_names.get(&key as &str))
							.map(|id| &field_types[*id])
						{
							Value::from_serde_by_type(self.context, value, key_type.clone())?
						} else {
							return Err(SerializerError::UnexpectedKey {
								r#type: self.expected_type.clone(),
								key: key.to_owned(),
							});
						}
					}
					_ => unreachable!(),
				};

				match result.entry(key.to_owned()) {
					Entry::Occupied(_) => Err(SerializerError::DuplicateKey {
						r#type: self.expected_type.clone(),
						key: key.to_owned(),
					}),
					Entry::Vacant(entry) => {
						entry.insert(value);
						Ok(())
					}
				}
			}
			_ => unreachable!(),
		}
	}

	fn serialize_field<T: ser::Serialize + ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), SerializerError> {
		match &mut self.variant {
			Variant::Struct {
				last_key,
				string_type: _,
				result,
			} => {
				assert!(last_key.is_none());

				let value = match self.expected_type.kind() {
					TypeKind::Struct => {
						let (field_names, field_types) = self.expected_type.fields();
						if let Some(key_type) = field_names
							.and_then(|field_names| field_names.get(&key as &str))
							.map(|id| &field_types[*id])
						{
							Value::from_serde_by_type(self.context, value, key_type.clone())?
						} else {
							return Err(SerializerError::UnexpectedKey {
								r#type: self.expected_type.clone(),
								key: key.into(),
							});
						}
					}
					_ => unreachable!(),
				};

				match result.entry(key.into()) {
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
			_ => unreachable!(),
		}
	}

	fn end(self) -> Result<Value, SerializerError> {
		match self.expected_type.kind() {
			TypeKind::Struct => {
				let map = match self.variant {
					Variant::Struct {
						last_key: _,
						string_type: _,
						result,
					} => result,
					_ => unreachable!(),
				};

				let value = Value::new_struct(self.context, self.expected_type.name(), map.into_iter())?;
				assert!(value.has_type(&self.expected_type));
				Ok(value)
			}
			_ => unreachable!(),
		}
	}
}

impl<'a> ser::SerializeMap for SerializeMap<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_key<T: ser::Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> {
		SerializeMap::serialize_key(self, key)
	}

	fn serialize_value<T: ser::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		SerializeMap::serialize_value(self, value)
	}

	fn serialize_entry<K: ser::Serialize + ?Sized, V: ser::Serialize + ?Sized>(
		&mut self,
		key: &K,
		value: &V,
	) -> Result<(), Self::Error> {
		Self::serialize_entry(self, key, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}

impl<'a> ser::SerializeStruct for SerializeMap<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_field<T: ser::Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		Self::serialize_field(self, key, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}

impl<'a> ser::SerializeStructVariant for SerializeMap<'a> {
	type Ok = Value;
	type Error = SerializerError;

	fn serialize_field<T: ser::Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		Self::serialize_field(self, key, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Self::end(self)
	}
}
