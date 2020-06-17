use num::BigInt;

use std::fmt;

use super::Data;
use Data::*;

impl fmt::Debug for Data {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Empty => f.debug_tuple("Empty").finish(),
			Bool(value) => f.debug_tuple("Bool").field(value).finish(),
			Integer(value) => f.debug_tuple("Integer").field(&IntegerFormatter(value)).finish(),
			Char(value) => f.debug_tuple("Char").field(value).finish(),
			String(value) => f.debug_tuple("String").field(value).finish(),
			UnitVariant(id) => f.debug_tuple("UnitVariant").field(id).finish(),
			Variant(id, value) => f.debug_tuple("Variant").field(id).field(&value).finish(),
			Array(value) => f.debug_tuple("Array").field(&value).finish(),
		}
	}
}

struct IntegerFormatter<'a>(&'a BigInt);

impl<'a> fmt::Debug for IntegerFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.0, f)
	}
}
