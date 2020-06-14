use std::collections::hash_map::Entry;
use std::sync::Arc;

use crate::error::{
	InvalidBoolType, InvalidCharType, InvalidIntegerType, InvalidStringType, InvalidTupleType, InvalidUnitType,
};
use crate::Type;

use super::Context;

impl Context {
	pub fn get_type(&self, key: &str) -> Option<Arc<dyn Type>> {
		self.0.types.read().unwrap().get(key).cloned()
	}

	pub fn get_unit_type(&self) -> Result<Arc<dyn Type>, InvalidUnitType> {
		self.get_type("()").ok_or_else(|| InvalidUnitType::Missing)
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
		if element_types.len() == 0 {
			return Err(InvalidTupleType::ZeroElements);
		}

		let name = to_tuple_name(element_types);
		if let Some(tuple_type) = self.get_type(&name) {
			return Ok(tuple_type);
		}

		// The fast path failed, we have to construct this tuple type. To ensure internal consistency, we have to claim a
		// write lock at this point, and can only release it once the newly created type is inserted.
		let mut types = self.0.types.write().unwrap();

		// Some other thread may have concurrently created this type in between our previous check and the acquisition of
		// the write lock, so we need to check again.
		if let Some(tuple_type) = types.get(&name) {
			return Ok(tuple_type.clone());
		}

		let element_types: Result<Vec<Arc<dyn Type>>, &str> = element_types
			.iter()
			.map(|&name| types.get(name).cloned().ok_or(name))
			.collect();
		if let Err(element_type_name) = element_types {
			return Err(InvalidTupleType::MissingSubtype {
				tuple_name: name,
				missing_subtype_name: element_type_name.to_string(),
			});
		}
		let element_types = element_types.unwrap();

		let tuple_type = Arc::new(crate::types::TupleType::new(name, element_types));
		assert!(types
			.insert(tuple_type.name().to_string(), tuple_type.clone())
			.is_none());

		Ok(tuple_type)
	}

	pub fn insert_type(&self, value: Arc<dyn Type>) {
		let name = value.name().to_string();
		let mut types = self.0.types.write().unwrap();
		match types.entry(name) {
			Entry::Occupied(_entry) => unimplemented!("Overwriting existing types is not supported (yet?)"),
			Entry::Vacant(entry) => {
				entry.insert(value);
			}
		}
	}

	pub fn remove_type(&self, key: &str) -> Option<(String, Arc<dyn Type>)> {
		let mut types = self.0.types.write().unwrap();
		let (key, value) = types.remove_entry(key)?;
		if Arc::strong_count(&value) == 1 {
			Some((key, value))
		} else {
			// There are still values or subtypes using this type, it cannot be removed now
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