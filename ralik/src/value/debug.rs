use num_bigint::BigInt;

use std::fmt;

use super::{Data, Value};

impl fmt::Debug for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.sign_plus() {
			f.debug_struct("Value")
				.field("type", &self.r#type)
				.field("data", &self.data)
				.finish()
		} else {
			f.debug_tuple("Value")
				.field(&self.r#type.name())
				.field(&self.data)
				.finish()
		}
	}
}

impl fmt::Debug for Data {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Data::*;

		if f.sign_plus() {
			match self {
				Unit => f.debug_tuple("Unit").finish(),
				Bool(value) => f.debug_tuple("Bool").field(value).finish(),
				Integer(value) => f.debug_tuple("Integer").field(&IntegerFormatter(value)).finish(),
				Char(value) => f.debug_tuple("Char").field(value).finish(),
				String(value) => f.debug_tuple("String").field(value).finish(),
				Tuple(value) => f.debug_tuple("Tuple").field(value).finish(),
				Struct(value) => f.debug_tuple("Struct").field(value).finish(),
				Array(value) => f.debug_tuple("Array").field(value).finish(),
			}
		} else {
			match self {
				Unit => ().fmt(f),
				Bool(value) => value.fmt(f),
				Char(value) => value.fmt(f),
				Integer(value) => IntegerFormatter(value).fmt(f),
				String(value) => value.fmt(f),
				Tuple(value) => f.debug_list().entries(value).finish(),
				Struct(value) => f.debug_list().entries(value.values()).finish(),
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
