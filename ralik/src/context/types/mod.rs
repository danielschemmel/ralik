use std::collections::hash_map::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::error::{InvalidBoolType, InvalidCharType, InvalidIntegerType, InvalidStringType, InvalidTupleType};
use crate::types::{Type, TypeBuilder, Variant, VariantBuilder};

use super::{Context, Thing, TypeId};

mod generics;
mod type_handle;
pub use type_handle::TypeHandle;

impl Context {
	pub fn get_type(&self, key: impl AsRef<str>) -> Option<TypeHandle> {
		let names = self.0.names.read().unwrap();
		match names.get(key.as_ref()) {
			Some(Thing::Type(id)) => Some(TypeHandle::from_type_id(self.clone(), *id)),
			_ => return None,
		}
	}

	pub fn get_unit_type(&self) -> Result<TypeHandle, InvalidTupleType> {
		unimplemented!()
	}

	pub fn get_bool_type(&self) -> Result<TypeHandle, InvalidBoolType> {
		self
			.get_type(crate::types::make_bool_name())
			.ok_or_else(|| InvalidBoolType::Missing)
	}

	pub fn get_char_type(&self) -> Result<TypeHandle, InvalidCharType> {
		self
			.get_type(crate::types::make_char_name())
			.ok_or_else(|| InvalidCharType::Missing)
	}

	pub fn get_integer_type(&self) -> Result<TypeHandle, InvalidIntegerType> {
		self
			.get_type(crate::types::make_integer_name())
			.ok_or_else(|| InvalidIntegerType::Missing)
	}

	pub fn get_string_type(&self) -> Result<TypeHandle, InvalidStringType> {
		self
			.get_type(crate::types::make_string_name())
			.ok_or_else(|| InvalidStringType::Missing)
	}

	pub fn register_types(&self, type_builders: Vec<TypeBuilder>) {
		let mut names = self.0.names.write().unwrap();

		for type_builder in &type_builders {
			if names.get(&type_builder.name).is_some() {
				panic!("Name {} is already in use", type_builder.name);
			}
		}

		let mut types = self.0.types.write().unwrap();

		let new_type_map = type_builders
			.iter()
			.map(|builder| builder.name.clone())
			.zip(
				types
					.iter()
					.map(|(_type, reference_count)| reference_count.load(Ordering::SeqCst) == 0)
					.chain(std::iter::repeat(true))
					.enumerate()
					.filter_map(|(index, free)| if free { Some(index) } else { None }),
			)
			.collect::<HashMap<String, usize>>();

		for (name, resolution) in type_builders
			.iter()
			.flat_map(|type_builder| {
				type_builder
					.type_parameters
					.iter()
					.chain(type_builder.field_types.iter())
					.chain(type_builder.variants.iter().flat_map(|variant| match variant {
						VariantBuilder::Unit(_name) => [].iter(),
						VariantBuilder::Tuple(_name, types) => types.iter(),
						VariantBuilder::Struct(_name, _names, types) => types.iter(),
					}))
			})
			.map(|type_name| (type_name, names.get(type_name)))
		{
			match resolution {
				Some(Thing::Type(_)) => (),
				Some(_) => panic!(
					"The dependent type {} resolved to something that is not actually a type",
					name
				),
				None => {
					if new_type_map.get(name).is_none() {
						panic!("The dependent type {} could not be resolved", name);
					}
				}
			}
		}

		// At this point, we have ensured that anything that we need later on is available

		for type_builder in type_builders {
			let id = *new_type_map.get(&type_builder.name).unwrap();
			if types.len() <= id {
				debug_assert!(types.len() == id);
				types.push((Default::default(), 1.into()));
			} else {
				let previous = types[id].1.fetch_add(1, Ordering::SeqCst);
				debug_assert!(previous == 0);
			}

			types[id].0 = Type {
				name: type_builder.name.into(),
				kind: type_builder.kind,
				type_parameters: type_builder
					.type_parameters
					.iter()
					.map(|name| {
						names
							.get(name)
							.map(|thing| match thing {
								Thing::Type(id) => *id,
								_ => unreachable!(),
							})
							.ok_or_else(|| new_type_map.get(name).unwrap())
							.unwrap()
					})
					.collect(),
				field_names: Arc::new(
					type_builder
						.field_names
						.into_iter()
						.map(|(name, index)| (name.into_boxed_str(), index))
						.collect(),
				),
				field_types: type_builder
					.field_types
					.iter()
					.map(|name| {
						names
							.get(name)
							.map(|thing| match thing {
								Thing::Type(id) => *id,
								_ => unreachable!(),
							})
							.ok_or_else(|| new_type_map.get(name).unwrap())
							.unwrap()
					})
					.collect(),
				variant_names: Arc::new(
					type_builder
						.variant_names
						.into_iter()
						.map(|(name, index)| (name.into_boxed_str(), index))
						.collect(),
				),
				variants: type_builder
					.variants
					.into_iter()
					.map(|variant_builder| match variant_builder {
						VariantBuilder::Unit(name) => Variant::Unit(name.into_boxed_str()),
						VariantBuilder::Tuple(name, field_types) => Variant::Tuple(
							name.into_boxed_str(),
							field_types
								.iter()
								.map(|name| {
									names
										.get(name)
										.map(|thing| match thing {
											Thing::Type(id) => *id,
											_ => unreachable!(),
										})
										.ok_or_else(|| new_type_map.get(name).unwrap())
										.unwrap()
								})
								.collect(),
						),
						VariantBuilder::Struct(name, field_names, field_types) => Variant::Struct(
							name.into_boxed_str(),
							field_names
								.into_iter()
								.map(|(name, index)| (name.into_boxed_str(), index))
								.collect(),
							field_types
								.iter()
								.map(|name| {
									names
										.get(name)
										.map(|thing| match thing {
											Thing::Type(id) => *id,
											_ => unreachable!(),
										})
										.ok_or_else(|| new_type_map.get(name).unwrap())
										.unwrap()
								})
								.collect(),
						),
					})
					.collect(),
				functions: Arc::new(
					type_builder
						.functions
						.into_iter()
						.map(|(name, function)| (name.into_boxed_str(), function))
						.collect(),
				),
			};

			let this_type = &types[id].0;
			for id in this_type
				.type_parameters
				.iter()
				.chain(this_type.field_types.iter())
				.chain(this_type.variants.iter().flat_map(|variant| match variant {
					Variant::Unit(_name) => [].iter(),
					Variant::Tuple(_name, types) => types.iter(),
					Variant::Struct(_name, _names, types) => types.iter(),
				})) {
				let previous = types[id.0].1.fetch_add(1, Ordering::SeqCst);
				if previous == isize::MAX || previous <= 0 {
					types[id.0].1.fetch_sub(1, Ordering::Relaxed);
					panic!("Reference count overflow!");
				}
			}

			names.insert((&*this_type.name).into(), Thing::Type(TypeId(id)));
		}
	}
}
