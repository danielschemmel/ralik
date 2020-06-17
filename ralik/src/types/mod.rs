use std::collections::hash_map::HashMap;

use crate::error::{Overflow, RuntimeError};
use crate::{Context, Value};

mod arguments;

mod array;
pub(crate) use self::array::name as array_name;
pub(crate) use self::array::ArrayType;

mod basic;
pub use basic::{BasicType, BasicTypeBase};

mod bool;
pub use self::bool::name as bool_name;
pub use self::bool::BoolType;

mod char;
pub use self::char::name as char_name;
pub use self::char::CharType;

mod r#enum;
pub use self::r#enum::EnumType;

mod integer;
pub use self::integer::name as integer_name;
pub use self::integer::IntegerType;

mod option;
pub(crate) use self::option::make_option_name;
pub(crate) use self::option::OptionType;

mod string;
pub use self::string::name as string_name;
pub use self::string::StringType;

mod r#struct;
pub use self::r#struct::StructType;

mod tuple;
pub(crate) use self::tuple::make_name as make_tuple_name;
pub(crate) use self::tuple::TupleType;

mod type_handle;
pub use self::type_handle::TypeHandle;

mod tuple_struct;
pub use self::tuple_struct::TupleStructType;

mod unit_struct;
pub use self::unit_struct::UnitStructType;

pub type MemberFunction = fn(&Context, &TypeHandle, &[Value]) -> Result<Value, RuntimeError>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TypeKind {
	Bool,
	Integer,
	Char,
	String,
	Tuple,
	UnitStruct,
	TupleStruct,
	Struct,
	Enum,
	Array,
}

#[derive(Clone, Debug)]
pub enum Variant {
	Unit(Box<str>),
	Tuple(Box<str>, Box<[TypeHandle]>),
	Struct(Box<str>, HashMap<Box<str>, usize>, Box<[TypeHandle]>),
}

impl Variant {
	pub fn name(&self) -> &str {
		match &self {
			Variant::Unit(name) => name,
			Variant::Tuple(name, _) => name,
			Variant::Struct(name, _, _) => name,
		}
	}
}

pub trait Type: std::fmt::Debug {
	fn name(&self) -> &str;
	fn kind(&self) -> TypeKind;

	fn type_parameters(&self) -> &[TypeHandle];
	fn fields(&self) -> (Option<&HashMap<Box<str>, usize>>, &[TypeHandle]);
	fn variants(&self) -> Option<(&HashMap<Box<str>, usize>, &[Variant])>;

	fn get_function(&self, key: &str) -> Option<&MemberFunction>;
	fn insert_function(&mut self, key: Box<str>, value: MemberFunction) -> Option<MemberFunction>;
	fn remove_function(&mut self, key: &str) -> Option<(Box<str>, MemberFunction)>;
}
