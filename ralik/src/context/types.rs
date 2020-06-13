use std::collections::hash_map::Entry;
use std::sync::Arc;

use crate::error::{InvalidBoolType, InvalidCharType, InvalidIntegerType, InvalidStringType, InvalidTupleType};
use crate::Type;

use super::Context;

impl Context {
	pub fn get_type(&self, key: &str) -> Option<Arc<dyn Type>> {
		self.0.types.read().unwrap().get(key).cloned()
	}

	pub fn get_bool_type(&self) -> Result<Arc<dyn Type>, InvalidBoolType> {
		self
			.get_type(crate::types::BoolName)
			.ok_or_else(|| InvalidBoolType::Missing)
	}

	pub fn get_char_type(&self) -> Result<Arc<dyn Type>, InvalidCharType> {
		self
			.get_type(crate::types::CharName)
			.ok_or_else(|| InvalidCharType::Missing)
	}

	pub fn get_integer_type(&self) -> Result<Arc<dyn Type>, InvalidIntegerType> {
		self
			.get_type(crate::types::IntegerName)
			.ok_or_else(|| InvalidIntegerType::Missing)
	}

	pub fn get_string_type(&self) -> Result<Arc<dyn Type>, InvalidStringType> {
		self
			.get_type(crate::types::StringName)
			.ok_or_else(|| InvalidStringType::Missing)
	}

	pub fn get_tuple_type(&self, element_types: &[&str]) -> Result<Arc<dyn Type>, InvalidTupleType> {
		let name = to_tuple_name(element_types);
		if let Some(tuple_type) = self.get_type(&name) {
			return Ok(tuple_type);
		}

		let mut resolved_element_types = Vec::with_capacity(element_types.len());
		for &element_type_name in element_types {
			let type_ref = self.get_type(element_type_name);
			if type_ref.is_none() {
				return Err(InvalidTupleType::MissingSubtype {
					tuple_name: name,
					missing_subtype_name: element_type_name.to_string(),
				});
			}
			let type_ref = type_ref.unwrap();
			resolved_element_types.push(type_ref.clone());
		}

		unimplemented!()
	}

	pub fn insert_type(&self, value: Arc<dyn Type>) -> bool {
		let name = value.name().to_string();
		let mut types = self.0.types.write().unwrap();
		match types.entry(name) {
			Entry::Occupied(entry) => {
				if Arc::strong_count(entry.get()) == 1 {
					entry.remove_entry();
					true
				} else {
					false
				}
			}
			Entry::Vacant(_) => false,
		}
	}

	pub fn remove_type(&self, key: &str) -> Option<(String, Arc<dyn Type>)> {
		let mut types = self.0.types.write().unwrap();
		let (key, value) = types.remove_entry(key)?;
		if Arc::strong_count(&value) == 1 {
			Some((key, value))
		} else {
			// There are still values using this type, it cannot be removed now
			assert!(types.insert(key, value).is_none());
			None
		}
	}
}

fn to_tuple_name(element_types: &[&str]) -> String {
	let mut name = "(".to_string();
	for (i, &element_type_name) in element_types.iter().enumerate() {
		if i > 0 {
			name.push_str(", ");
		}
		name.push_str(element_type_name);
	}
	name.push_str(")");

	name
}
