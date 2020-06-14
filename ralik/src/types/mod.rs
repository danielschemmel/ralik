use std::sync::Arc;

use crate::error::{Overflow, RuntimeError};
use crate::{Context, Value};

mod arguments;

mod array;
pub(crate) use self::array::ArrayType;
pub use self::array::name as array_name;

mod basic;
pub use basic::{BasicFunctionStore, BasicType, BasicTypeBase};

mod bool;
pub use self::bool::BoolType;
pub use self::bool::name as bool_name;

mod char;
pub use self::char::CharType;
pub use self::char::name as char_name;

mod debug;

mod integer;
pub use self::integer::IntegerType;
pub use self::integer::name as integer_name;

mod string;
pub use self::string::StringType;
pub use self::string::name as string_name;

mod tuple;
pub(crate) use self::tuple::TupleType;
pub use self::tuple::name as tuple_name;

mod unit;
pub use self::unit::UnitType;
pub use self::unit::name as unit_name;

pub type MemberFunction = fn(&Context, &Arc<dyn Type>, &[Value]) -> Result<Value, RuntimeError>;

pub trait Type: std::fmt::Debug {
	fn name(&self) -> &str;

	fn get_function(&self, key: &str) -> Option<&MemberFunction>;
	fn insert_function(&mut self, key: String, value: MemberFunction) -> Option<MemberFunction>;
	fn remove_function(&mut self, key: &str) -> Option<(String, MemberFunction)>;
}
