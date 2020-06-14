use std::collections::hash_map::Entry;
use std::sync::Arc;

use crate::error::{
	InvalidArrayType, InvalidBoolType, InvalidCharType, InvalidIntegerType, InvalidStringType, InvalidTupleType,
	InvalidUnitType,
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
			.get_type(crate::types::bool_name())
			.ok_or_else(|| InvalidBoolType::Missing)
	}

	pub fn get_char_type(&self) -> Result<Arc<dyn Type>, InvalidCharType> {
		self
			.get_type(crate::types::char_name())
			.ok_or_else(|| InvalidCharType::Missing)
	}

	pub fn get_integer_type(&self) -> Result<Arc<dyn Type>, InvalidIntegerType> {
		self
			.get_type(crate::types::integer_name())
			.ok_or_else(|| InvalidIntegerType::Missing)
	}

	pub fn get_string_type(&self) -> Result<Arc<dyn Type>, InvalidStringType> {
		self
			.get_type(crate::types::string_name())
			.ok_or_else(|| InvalidStringType::Missing)
	}

	pub fn get_tuple_type(&self, element_type_names: &[&str]) -> Result<Arc<dyn Type>, InvalidTupleType> {
		if element_type_names.len() == 0 {
			return Err(InvalidTupleType::ZeroElements);
		}

		let name = crate::types::tuple_name(element_type_names);
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

		let element_types: Result<Vec<Arc<dyn Type>>, &str> = element_type_names
			.iter()
			.map(|&name| types.get(name).cloned().ok_or(name))
			.collect();
		if let Err(element_type_name) = element_types {
			return Err(InvalidTupleType::MissingSubtype {
				tuple_name: name,
				missing_element_type_name: element_type_name.to_string(),
			});
		}
		let element_types = element_types.unwrap();

		let tuple_type = Arc::new(crate::types::TupleType::new(name, element_types));
		assert!(types
			.insert(tuple_type.name().to_string(), tuple_type.clone())
			.is_none());

		Ok(tuple_type)
	}

	pub fn get_array_type(&self, element_type_name: &str) -> Result<Arc<dyn Type>, InvalidArrayType> {
		let name = crate::types::array_name(element_type_name);
		if let Some(array_type) = self.get_type(&name) {
			return Ok(array_type);
		}

		// The fast path failed, we have to construct this tuple type. To ensure internal consistency, we have to claim a
		// write lock at this point, and can only release it once the newly created type is inserted.
		let mut types = self.0.types.write().unwrap();

		// Some other thread may have concurrently created this type in between our previous check and the acquisition of
		// the write lock, so we need to check again.
		if let Some(array_type) = types.get(&name) {
			return Ok(array_type.clone());
		}

		let element_type = types
			.get(element_type_name)
			.ok_or_else(|| InvalidArrayType::MissingSubtype {
				element_type_name: element_type_name.to_string(),
			})?;

		let array_type = crate::types::ArrayType::new(name, element_type.clone());
		assert!(types
			.insert(array_type.name().to_string(), array_type.clone())
			.is_none());

		Ok(array_type)
	}

	pub fn insert_type(&self, value: Arc<dyn Type>) {
		let name = value.name().to_string();
		let mut types = self.0.types.write().unwrap();
		match types.entry(name) {
			Entry::Occupied(_entry) => panic!("Overwriting existing types is not supported (yet?)"),
			Entry::Vacant(entry) => {
				entry.insert(value);
			}
		}
	}

	pub fn remove_type(&self, key: &str) {
		let mut types = self.0.types.write().unwrap();
		let (owned_key, weak_ref) = {
			if let Some((key, value)) = types.remove_entry(key) {
				(key, Arc::downgrade(&value))
			} else {
				// type does not exist?
				panic!("Type {} does not exist in context", key);
			}
		};
		if let Some(value) = weak_ref.upgrade() {
			// type is still in use somewhere else, or it would not have been possible to upgrade it again
			types.insert(owned_key, value);
			panic!("Type {} is still in use", key);
		}
	}
}
