use num::BigInt;

use crate::error::{
	ArrayCreationError, BoolCreationError, CharCreationError, EnumStructVariantCreationError,
	EnumTupleVariantCreationError, EnumUnitVariantCreationError, IntegerCreationError, InvalidArrayType,
	StringCreationError, StructCreationError, TupleCreationError, TupleStructCreationError, UnitStructCreationError,
};
use crate::types::{TypeKind, Variant};
use crate::{Context, TypeHandle};

use super::{Data, Value};

impl Value {
	pub fn new_unit(context: &Context) -> Result<Value, TupleCreationError> {
		Ok(Value {
			r#type: context.get_unit_type()?.clone(),
			data: Data::Empty,
		})
	}

	pub fn new_bool(context: &Context, value: impl Into<bool>) -> Result<Value, BoolCreationError> {
		Ok(Value {
			r#type: context.get_bool_type()?.clone(),
			data: Data::Bool(value.into()),
		})
	}

	pub fn new_char(context: &Context, value: impl Into<char>) -> Result<Value, CharCreationError> {
		Ok(Value {
			r#type: context.get_char_type()?.clone(),
			data: Data::Char(value.into()),
		})
	}

	pub fn new_integer(context: &Context, value: impl Into<BigInt>) -> Result<Value, IntegerCreationError> {
		Ok(Value {
			r#type: context.get_integer_type()?.clone(),
			data: Data::Integer(value.into()),
		})
	}

	pub fn new_string(context: &Context, value: impl Into<Box<str>>) -> Result<Value, StringCreationError> {
		Ok(Value {
			r#type: context.get_string_type()?.clone(),
			data: Data::String(value.into()),
		})
	}

	pub fn new_tuple(context: &Context, values: impl Into<Box<[Value]>>) -> Result<Value, TupleCreationError> {
		let values: Box<[Value]> = values.into();
		let element_types = values.iter().map(|value| value.get_type().name());
		let tuple_type = context.get_tuple_type(element_types)?.clone();
		Ok(Value {
			r#type: tuple_type,
			data: Data::Array(values),
		})
	}

	pub fn new_tuple_struct(
		context: &Context,
		name: impl AsRef<str>,
		values: impl Into<Box<[Value]>>,
	) -> Result<Value, TupleStructCreationError> {
		let tuple_type = context
			.get_type(name.as_ref())
			.ok_or_else(|| crate::error::InvalidTupleStructType::Missing {
				type_name: name.as_ref().into(),
			})?
			.clone();
		if tuple_type.kind() != TypeKind::TupleStruct {
			return Err(crate::error::InvalidTupleStructType::NotTupleStructType { r#type: tuple_type }.into());
		}

		let values: Box<[Value]> = values.into();
		let element_types = tuple_type.fields().1;
		if values.len() != element_types.len() {
			return Err(TupleStructCreationError::ElementCount {
				type_element_count: element_types.len(),
				provided_element_count: values.len(),
			});
		}

		if values.is_empty() {
			Ok(Value {
				r#type: tuple_type,
				data: Data::Empty,
			})
		} else {
			for (index, (value, expected_type)) in values.iter().zip(element_types.iter()).enumerate() {
				let value_type = value.get_type();
				if !TypeHandle::is_same(value_type, expected_type) {
					return Err(TupleStructCreationError::ElementTypeMismatch {
						index,
						expected: expected_type.clone(),
						actual: value_type.clone(),
					});
				}
			}

			Ok(Value {
				r#type: tuple_type,
				data: Data::Array(values),
			})
		}
	}

	pub fn new_struct(
		context: &Context,
		name: impl AsRef<str>,
		mut fields: impl Iterator<Item = (impl AsRef<str>, Value)>,
	) -> Result<Value, StructCreationError> {
		let name = name.as_ref();

		let struct_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidStructType::Missing { type_name: name.into() })?
			.clone();
		if struct_type.kind() != TypeKind::Struct {
			return Err(crate::error::InvalidStructType::NotStructType { r#type: struct_type }.into());
		}

		let (field_names, field_types) = struct_type.fields();

		if field_types.is_empty() {
			if let Some((field_name, _field_value)) = fields.next() {
				Err(StructCreationError::SuperfluousField {
					r#type: struct_type,
					field_name: field_name.as_ref().into(),
				})
			} else {
				Ok(Value {
					r#type: struct_type,
					data: Data::Empty,
				})
			}
		} else {
			if field_names.is_none() {
				return Err(crate::error::InvalidStructType::NoFieldNames { r#type: struct_type }.into());
			}
			let field_names = field_names.unwrap();

			let fields = fields
				.map(|(field_name, value)| {
					if let Some(index) = field_names.get(field_name.as_ref()) {
						Ok((*index, field_name, value))
					} else {
						Err(field_name.as_ref().into())
					}
				})
				.collect::<Result<Vec<(usize, _, Value)>, String>>();

			let mut fields = match fields {
				Ok(fields) => fields,
				Err(field_name) => {
					return Err(StructCreationError::SuperfluousField {
						r#type: struct_type.clone(),
						field_name,
					})
				}
			};

			fields.sort_unstable_by_key(|(key, _name, _value)| *key);

			let fields = fields
				.into_iter()
				.enumerate()
				.map(|(i, (key, name, value))| {
					if i < key {
						Err(StructCreationError::MissingField {
							r#type: struct_type.clone(),
							field_name: name.as_ref().into(),
						})
					} else if i > key {
						Err(StructCreationError::DuplicateField {
							r#type: struct_type.clone(),
							field_name: name.as_ref().into(),
						})
					} else if !value.has_type(&field_types[key]) {
						Err(StructCreationError::FieldTypeMismatch {
							r#type: struct_type.clone(),
							field_name: name.as_ref().into(),
							field_type: field_types[key].clone(),
							value_type: value.get_type().clone(),
						})
					} else {
						Ok(value)
					}
				})
				.collect::<Result<Vec<Value>, StructCreationError>>()?;

			Ok(Value {
				r#type: struct_type,
				data: Data::Array(fields.into_boxed_slice()),
			})
		}
	}

	pub fn new_unit_struct(context: &Context, name: impl AsRef<str>) -> Result<Value, UnitStructCreationError> {
		let name = name.as_ref();

		let struct_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidUnitStructType::Missing { type_name: name.into() })?
			.clone();
		if struct_type.kind() != TypeKind::UnitStruct {
			return Err(crate::error::InvalidUnitStructType::NotUnitStructType { r#type: struct_type }.into());
		}

		Ok(Value {
			r#type: struct_type,
			data: Data::Empty,
		})
	}

	pub fn new_array(
		context: &Context,
		element_type: &TypeHandle,
		values: impl Into<Box<[Value]>>,
	) -> Result<Value, ArrayCreationError> {
		let values: Box<[Value]> = values.into();

		if let Some((index, value)) = values
			.iter()
			.enumerate()
			.find(|(_index, value)| !value.has_type(element_type))
		{
			return Err(
				InvalidArrayType::InvalidElement {
					value: value.clone(),
					index,
					type_name: crate::types::array_name(element_type.name()),
				}
				.into(),
			);
		}

		let array_type = context.get_array_type(element_type.name())?.into();
		Ok(Value {
			r#type: array_type,
			data: Data::Array(values),
		})
	}

	pub fn new_enum_unit_variant(
		context: &Context,
		name: impl AsRef<str>,
		variant_name: impl AsRef<str>,
	) -> Result<Value, EnumUnitVariantCreationError> {
		let name = name.as_ref();
		let enum_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidEnumType::Missing { type_name: name.into() })?
			.clone();
		if enum_type.kind() != TypeKind::Enum {
			return Err(crate::error::InvalidEnumType::NotEnumType { r#type: enum_type }.into());
		}

		let variant_name = variant_name.as_ref();
		let (variant_names, variant_ids) = if let Some(variants) = enum_type.variants() {
			variants
		} else {
			return Err(EnumUnitVariantCreationError::VariantMissing {
				r#type: enum_type,
				variant_name: variant_name.into(),
			});
		};

		let variant_id = if let Some(variant_id) = variant_names.get(variant_name) {
			*variant_id
		} else {
			return Err(EnumUnitVariantCreationError::VariantMissing {
				r#type: enum_type,
				variant_name: variant_name.into(),
			});
		};

		match &variant_ids[variant_id] {
			Variant::Unit(_name) => Ok(Value {
				r#type: enum_type,
				data: Data::UnitVariant(variant_id),
			}),
			_ => Err(EnumUnitVariantCreationError::NotUnitVariant {
				r#type: enum_type,
				variant_name: variant_name.into(),
			}),
		}
	}

	pub fn new_enum_tuple_variant(
		context: &Context,
		name: impl AsRef<str>,
		variant_name: impl AsRef<str>,
		values: impl Into<Box<[Value]>>,
	) -> Result<Value, EnumTupleVariantCreationError> {
		let name = name.as_ref();
		let enum_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidEnumType::Missing { type_name: name.into() })?
			.clone();
		if enum_type.kind() != TypeKind::Enum {
			return Err(crate::error::InvalidEnumType::NotEnumType { r#type: enum_type }.into());
		}

		let variant_name = variant_name.as_ref();
		let (variant_names, variant_ids) = if let Some(variants) = enum_type.variants() {
			variants
		} else {
			return Err(EnumTupleVariantCreationError::VariantMissing {
				r#type: enum_type,
				variant_name: variant_name.into(),
			});
		};

		let variant_id = if let Some(variant_id) = variant_names.get(variant_name) {
			*variant_id
		} else {
			return Err(EnumTupleVariantCreationError::VariantMissing {
				r#type: enum_type,
				variant_name: variant_name.into(),
			});
		};

		let values = values.into();

		match &variant_ids[variant_id] {
			Variant::Tuple(_name, element_types) => {
				for (index, (value, expected_type)) in values.iter().zip(element_types.iter()).enumerate() {
					let value_type = value.get_type();
					if !TypeHandle::is_same(value_type, expected_type) {
						return Err(EnumTupleVariantCreationError::ElementTypeMismatch {
							index,
							expected: expected_type.clone(),
							actual: value_type.clone(),
						});
					}
				}

				Ok(Value {
					r#type: enum_type,
					data: Data::Variant(variant_id, values),
				})
			}
			_ => Err(EnumTupleVariantCreationError::NotTupleVariant {
				r#type: enum_type,
				variant_name: variant_name.into(),
			}),
		}
	}

	pub fn new_enum_struct_variant(
		context: &Context,
		name: impl AsRef<str>,
		variant_name: impl AsRef<str>,
		mut fields: impl Iterator<Item = (impl AsRef<str>, Value)>,
	) -> Result<Value, EnumStructVariantCreationError> {
		let name = name.as_ref();
		let enum_type = context
			.get_type(name)
			.ok_or_else(|| crate::error::InvalidEnumType::Missing { type_name: name.into() })?
			.clone();
		if enum_type.kind() != TypeKind::Enum {
			return Err(crate::error::InvalidEnumType::NotEnumType { r#type: enum_type }.into());
		}

		let variant_name = variant_name.as_ref();
		let (variant_names, variant_ids) = if let Some(variants) = enum_type.variants() {
			variants
		} else {
			return Err(EnumStructVariantCreationError::VariantMissing {
				r#type: enum_type,
				variant_name: variant_name.into(),
			});
		};

		let variant_id = if let Some(variant_id) = variant_names.get(variant_name) {
			*variant_id
		} else {
			return Err(EnumStructVariantCreationError::VariantMissing {
				r#type: enum_type,
				variant_name: variant_name.into(),
			});
		};

		match &variant_ids[variant_id] {
			Variant::Struct(_name, field_names, field_types) => {
				if field_types.is_empty() {
					if let Some((field_name, _field_value)) = fields.next() {
						Err(EnumStructVariantCreationError::SuperfluousField {
							r#type: enum_type,
							field_name: field_name.as_ref().into(),
						})
					} else {
						Ok(Value {
							r#type: enum_type,
							data: Data::UnitVariant(variant_id),
						})
					}
				} else {
					let fields = fields
						.map(|(field_name, value)| {
							if let Some(index) = field_names.get(field_name.as_ref()) {
								Ok((*index, field_name, value))
							} else {
								Err(field_name.as_ref().into())
							}
						})
						.collect::<Result<Vec<(usize, _, Value)>, String>>();

					let mut fields = match fields {
						Ok(fields) => fields,
						Err(field_name) => {
							return Err(EnumStructVariantCreationError::SuperfluousField {
								r#type: enum_type.clone(),
								field_name,
							})
						}
					};

					fields.sort_unstable_by_key(|(key, _name, _value)| *key);

					let fields = fields
						.into_iter()
						.enumerate()
						.map(|(i, (key, name, value))| {
							if i < key {
								Err(EnumStructVariantCreationError::MissingField {
									r#type: enum_type.clone(),
									field_name: name.as_ref().into(),
								})
							} else if i > key {
								Err(EnumStructVariantCreationError::DuplicateField {
									r#type: enum_type.clone(),
									field_name: name.as_ref().into(),
								})
							} else if !value.has_type(&field_types[key]) {
								Err(EnumStructVariantCreationError::FieldTypeMismatch {
									r#type: enum_type.clone(),
									field_name: name.as_ref().into(),
									field_type: field_types[key].clone(),
									value_type: value.get_type().clone(),
								})
							} else {
								Ok(value)
							}
						})
						.collect::<Result<Vec<Value>, EnumStructVariantCreationError>>()?;

					Ok(Value {
						r#type: enum_type,
						data: Data::Variant(variant_id, fields.into_boxed_slice()),
					})
				}
			}
			_ => Err(EnumStructVariantCreationError::NotStructVariant {
				r#type: enum_type,
				variant_name: variant_name.into(),
			}),
		}
	}
}
