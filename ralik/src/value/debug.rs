use num_bigint::BigInt;

use std::fmt;

use super::{Data, Value};
use Data::*;

impl fmt::Debug for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.sign_plus() {
			let mut builder = f.debug_struct("Value");
			builder.field("type", &self.r#type);
			match &self.data {
				EnumUnit(name) | EnumTuple(name, _) | EnumStruct(name, _) => {
					builder.field("variant", &name);
				}
				_ => {}
			}
			builder.field("data", &self.data).finish()
		} else {
			let mut builder = f.debug_tuple("Value");
			builder.field(&self.r#type.name());
			match &self.data {
				EnumUnit(name) | EnumTuple(name, _) | EnumStruct(name, _) => {
					builder.field(&name);
				}
				_ => {}
			}
			builder.field(&self.data).finish()
		}
	}
}

impl fmt::Debug for Data {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.sign_plus() {
			match self {
				Unit => f.debug_tuple("Unit").finish(),
				Bool(value) => f.debug_tuple("Bool").field(value).finish(),
				Integer(value) => f.debug_tuple("Integer").field(&IntegerFormatter(value)).finish(),
				Char(value) => f.debug_tuple("Char").field(value).finish(),
				String(value) => f.debug_tuple("String").field(value).finish(),
				Tuple(value) => f.debug_tuple("Tuple").field(value).finish(),
				Struct(value) => f.debug_tuple("Struct").field(value).finish(),
				EnumUnit(name) => f.debug_tuple("EnumUnit").field(name).finish(),
				EnumTuple(name, value) => f.debug_tuple("EnumTuple").field(name).field(value).finish(),
				EnumStruct(name, value) => f.debug_tuple("EnumStruct").field(name).field(value).finish(),
				Array(value) => f.debug_tuple("Array").field(value).finish(),
			}
		} else {
			match self {
				Unit | EnumUnit(_) => ().fmt(f),
				Bool(value) => value.fmt(f),
				Char(value) => value.fmt(f),
				Integer(value) => IntegerFormatter(value).fmt(f),
				String(value) => value.fmt(f),
				Tuple(value) | EnumTuple(_, value) => f.debug_list().entries(value).finish(),
				Struct(value) | EnumStruct(_, value) => f.debug_list().entries(value.values()).finish(),
				Array(value) => f.debug_list().entries(value).finish(),
			}
		}
	}
}

struct IntegerFormatter<'a>(&'a BigInt);

impl<'a> fmt::Debug for IntegerFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.0, f)
	}
}
