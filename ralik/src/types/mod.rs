use pretty_assertions::assert_ne;

use crate::error::{
	InvalidStructType, InvalidTupleType, Overflow, RuntimeError, StructCreationError, TupleCreationError,
};
use crate::{Context, Value};

mod arguments;

mod array;
pub use self::array::name as array_name;
pub(crate) use self::array::ArrayType;

mod basic;
pub use basic::{BasicFunctionStore, BasicType, BasicTypeBase};

mod bool;
pub use self::bool::name as bool_name;
pub use self::bool::BoolType;

mod char;
pub use self::char::name as char_name;
pub use self::char::CharType;

mod debug;

mod integer;
pub use self::integer::name as integer_name;
pub use self::integer::IntegerType;

mod string;
pub use self::string::name as string_name;
pub use self::string::StringType;

mod tuple;
pub use self::tuple::name as tuple_name;
pub(crate) use self::tuple::TupleType;

mod type_handle;
pub use self::type_handle::TypeHandle;

mod r#struct;
pub use self::r#struct::StructType;

mod unit;
pub use self::unit::name as unit_name;
pub use self::unit::UnitType;

pub type MemberFunction = fn(&Context, &TypeHandle, &[Value]) -> Result<Value, RuntimeError>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TypeKind {
	Unit,
	Bool,
	Integer,
	Char,
	String,
	Tuple,
	Struct,
	Array,
}

pub trait Type: std::fmt::Debug {
	fn name(&self) -> &str;
	fn kind(&self) -> TypeKind;

	fn construct_struct(
		&self,
		#[allow(unused_variables)] fields: &[(String, Value)],
	) -> Result<Value, StructCreationError> {
		assert_ne!(self.kind(), TypeKind::Struct);

		Err(
			InvalidStructType::NotStructType {
				type_name: self.name().to_string(),
			}
			.into(),
		)
	}

	fn construct_tuple(&self, #[allow(unused_variables)] fields: &[Value]) -> Result<Value, TupleCreationError> {
		assert_ne!(self.kind(), TypeKind::Tuple);

		Err(
			InvalidTupleType::NotTupleType {
				type_name: self.name().to_string(),
			}
			.into(),
		)
	}

	fn get_function(&self, key: &str) -> Option<&MemberFunction>;
	fn insert_function(&mut self, key: String, value: MemberFunction) -> Option<MemberFunction>;
	fn remove_function(&mut self, key: &str) -> Option<(String, MemberFunction)>;
}
