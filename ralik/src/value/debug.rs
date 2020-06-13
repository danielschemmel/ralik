use num_bigint::BigInt;

use std::fmt;

impl fmt::Debug for super::Data {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use super::Data::*;
		match self {
			Bool(value) => f.debug_tuple("Bool").field(value).finish(),
			Char(value) => f.debug_tuple("Char").field(value).finish(),
			Integer(value) => f.debug_tuple("Integer").field(&IntegerFormatter(value)).finish(),
			String(value) => f.debug_tuple("String").field(value).finish(),
			Vec(value) => f.debug_tuple("Vec").field(&value).finish(),
		}
	}
}

struct IntegerFormatter<'a>(&'a BigInt);

impl<'a> fmt::Debug for IntegerFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.0, f)
	}
}
