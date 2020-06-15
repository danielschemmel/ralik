use crate::error::{
	InvalidArrayType, InvalidBoolType, InvalidCharType, InvalidIntegerType, InvalidStringType, InvalidTupleType,
	InvalidUnitType,
};
use crate::{Type, TypeHandle};

use super::Context;

mod type_container;
pub(super) use type_container::TypeContainer;

impl Context {
	pub fn get_type(&self, key: &str) -> Option<TypeHandle> {
		self.0.types.data.read().unwrap().get(key).map(TypeHandle::from)
	}

	pub fn get_unit_type(&self) -> Result<TypeHandle, InvalidUnitType> {
		self.get_type("()").ok_or_else(|| InvalidUnitType::Missing)
	}

	pub fn get_bool_type(&self) -> Result<TypeHandle, InvalidBoolType> {
		self
			.get_type(crate::types::bool_name())
			.ok_or_else(|| InvalidBoolType::Missing)
	}

	pub fn get_char_type(&self) -> Result<TypeHandle, InvalidCharType> {
		self
			.get_type(crate::types::char_name())
			.ok_or_else(|| InvalidCharType::Missing)
	}

	pub fn get_integer_type(&self) -> Result<TypeHandle, InvalidIntegerType> {
		self
			.get_type(crate::types::integer_name())
			.ok_or_else(|| InvalidIntegerType::Missing)
	}

	pub fn get_string_type(&self) -> Result<TypeHandle, InvalidStringType> {
		self
			.get_type(crate::types::string_name())
			.ok_or_else(|| InvalidStringType::Missing)
	}

	pub fn get_tuple_type(&self, element_type_names: &[&str]) -> Result<TypeHandle, InvalidTupleType> {
		if element_type_names.len() == 0 {
			return Err(InvalidTupleType::ZeroElements);
		}

		let name = crate::types::tuple_name(element_type_names);
		if let Some(tuple_type) = self.get_type(&name) {
			return Ok(tuple_type);
		}

		// The fast path failed, we have to construct this tuple type. To ensure internal consistency, we have to claim a
		// write lock at this point, and can only release it once the newly created type is inserted.
		let mut types = self.0.types.data.write().unwrap();

		// Some other thread may have concurrently created this type in between our previous check and the acquisition of
		// the write lock, so we need to check again.
		if let Some(tuple_type) = types.get(name.as_str()) {
			return Ok(tuple_type.into());
		}

		let element_types: Result<Vec<TypeHandle>, &str> = element_type_names
			.iter()
			.map(|&name| types.get(name).map(TypeHandle::from).ok_or(name))
			.collect();
		if let Err(element_type_name) = element_types {
			return Err(InvalidTupleType::MissingSubtype {
				tuple_name: name,
				missing_element_type_name: element_type_name.into(),
			});
		}
		let element_types = element_types.unwrap();

		let tuple_type = TypeHandle::new(crate::types::TupleType::new(name, element_types));
		assert!(types.insert(tuple_type.clone().into()) == true);

		Ok(tuple_type)
	}

	pub fn get_array_type(&self, element_type_name: &str) -> Result<TypeHandle, InvalidArrayType> {
		let name = crate::types::array_name(element_type_name);
		if let Some(array_type) = self.get_type(&name) {
			return Ok(array_type);
		}

		// The fast path failed, we have to construct this tuple type. To ensure internal consistency, we have to claim a
		// write lock at this point, and can only release it once the newly created type is inserted.
		let mut types = self.0.types.data.write().unwrap();

		// Some other thread may have concurrently created this type in between our previous check and the acquisition of
		// the write lock, so we need to check again.
		if let Some(array_type) = types.get(name.as_str()) {
			return Ok(array_type.into());
		}

		let element_type =
			types
				.get(element_type_name)
				.map(TypeHandle::from)
				.ok_or_else(|| InvalidArrayType::MissingSubtype {
					element_type_name: element_type_name.into(),
				})?;

		let array_type = TypeHandle::new(crate::types::ArrayType::new(name, element_type));
		assert!(types.insert(array_type.clone().into()) == true);

		Ok(array_type)
	}

	pub fn insert_type(&self, value: impl Type + 'static) -> TypeHandle {
		let handle = TypeHandle::new(value);

		// check for consistency
		{
			let types = self.0.types.data.read().unwrap();
			let type_parameters = handle.type_parameters().iter();
			let field_types = handle.fields().map(|fields| fields.values()).into_iter().flatten();
			use crate::types::Variant;
			let variant_types = handle
				.variants()
				.map(|fields| fields.values())
				.into_iter()
				.flatten()
				.flat_map(|variant| {
					let iterator: Box<dyn Iterator<Item = &TypeHandle>> = match variant {
						Variant::Unit => Box::new(None.iter()),
						Variant::Tuple(types) => Box::new(types.iter()),
						Variant::Struct(map) => Box::new(map.values()),
					};
					iterator
				});

			for type_parameter in type_parameters.chain(field_types).chain(variant_types) {
				let registered = types.get(type_parameter.name());
				assert!(registered.is_some(), "All dependent types must be registered first");
				assert!(
					TypeHandle::is_same(type_parameter, registered.unwrap().into()),
					"All dependent types must refer to the exact same object that is registered under that name"
				);
			}
		}

		let mut types = self.0.types.data.write().unwrap();
		assert!(types.insert(handle.clone().into()) == true);
		handle
	}
}
