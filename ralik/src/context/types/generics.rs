use crate::error::{InvalidArrayType, InvalidTupleType};
use crate::types::TypeBuilder;

use super::super::{Context, GenericTypeCreator, Thing, TypeHandle};

impl Context {
	pub fn register_tuple_generic(&self, tuple_generic: GenericTypeCreator) {
		let mut tuples = self.0.tuples.write().unwrap();
		if tuples.is_some() {
			panic!("Replacing the array generic is not supported");
		}
		*tuples = Some(tuple_generic);
	}

	pub fn register_array_generic(&self, array_generic: GenericTypeCreator) {
		let mut arrays = self.0.arrays.write().unwrap();
		if arrays.is_some() {
			panic!("Replacing the array generic is not supported");
		}
		*arrays = Some(array_generic);
	}

	pub fn get_array_type(&self, element_type_name: &str) -> Result<TypeHandle, InvalidArrayType> {
		let name = crate::types::make_array_name(element_type_name);
		if let Some(array_type) = self.get_type(&name) {
			return Ok(array_type);
		}

		let generic_type_build = match self
			.0
			.arrays
			.read()
			.unwrap()
			.ok_or_else(|| InvalidArrayType::MissingGeneric)?(self, &[element_type_name])
		{
			Ok(result) => result,
			Err(error) => return Err(InvalidArrayType::GenericFailed { name, error }),
		};

		self.register_types(vec![TypeBuilder::from_generic_type_builder(&name, generic_type_build)]);
		Ok(self.get_type(&name).unwrap())
	}

	pub fn get_tuple_type(&self, element_type_names: Vec<impl AsRef<str>>) -> Result<TypeHandle, InvalidTupleType> {
		let name = crate::types::make_tuple_name(element_type_names.iter().map(|name| name.as_ref()));
		if let Some(array_type) = self.get_type(&name) {
			return Ok(array_type);
		}

		let element_type_names = element_type_names.iter().map(|name| name.as_ref()).collect::<Vec<_>>();
		let generic_type_build = match self
			.0
			.arrays
			.read()
			.unwrap()
			.ok_or_else(|| InvalidTupleType::MissingGeneric)?(self, &element_type_names)
		{
			Ok(result) => result,
			Err(error) => return Err(InvalidTupleType::GenericFailed { name, error }),
		};

		self.register_types(vec![TypeBuilder::from_generic_type_builder(&name, generic_type_build)]);
		Ok(self.get_type(&name).unwrap())
	}

	pub fn get_generic_type(&self, name: impl AsRef<str>, type_parameters: Vec<impl AsRef<str>>) -> TypeHandle {
		{
			// fast path
			let names = self.0.names.read().unwrap();
			let generic = match names.get(name.as_ref()) {
				Some(Thing::Generic(generic)) => generic.clone(),
				_ => panic!("The generic {} is not registered", name.as_ref()),
			};

			let mut canonical_name = format!("{}<", generic.name);
			for param in &type_parameters {
				canonical_name.push_str(param.as_ref());
			}
			canonical_name.push_str(">");

			match names.get(&canonical_name) {
				Some(Thing::Type(id)) => return TypeHandle::from_type_id(self.clone(), *id),
				_ => {}
			}
		}

		unimplemented!()
	}
}
