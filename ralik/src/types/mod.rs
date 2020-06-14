use crate::error::{Overflow, RuntimeError};
use crate::{Context, Value};

mod arguments;

mod basic;
pub use basic::{BasicFunctionStore, BasicType, BasicTypeBase};

mod bool;
pub use self::bool::BoolType;
pub use self::bool::NAME as BoolName;

mod char;
pub use self::char::CharType;
pub use self::char::NAME as CharName;

mod debug;

mod integer;
pub use self::integer::IntegerType;
pub use self::integer::NAME as IntegerName;

mod string;
pub use self::string::StringType;
pub use self::string::NAME as StringName;

mod tuple;
pub(crate) use self::tuple::TupleType;

mod unit;
pub use self::unit::UnitType;
pub use self::unit::NAME as UnitName;

pub type MemberFunction = fn(&Context, &[Value]) -> Result<Value, RuntimeError>;

pub trait Type: std::fmt::Debug {
	fn name(&self) -> &str;

	fn get_function(&self, key: &str) -> Option<&MemberFunction>;
	fn insert_function(&mut self, key: String, value: MemberFunction) -> Option<MemberFunction>;
	fn remove_function(&mut self, key: &str) -> Option<(String, MemberFunction)>;
}
