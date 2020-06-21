use std::collections::hash_map::HashMap;
use std::sync::Arc;

use crate::context::{TypeHandle, TypeId};
use crate::error::RuntimeError;
use crate::{Context, Value};

mod arguments;

mod array;
pub use self::array::array_generic;
pub(crate) use self::array::make_array_name;

mod bool;
pub use self::bool::{make_bool_name, new_bool_type};

mod char;
pub use self::char::{make_char_name, new_char_type};

mod r#enum;
pub use self::r#enum::new_enum_type;

mod integer;
pub use self::integer::{make_integer_name, new_integer_type};

//mod option;
//pub(crate) use self::option::make_option_name;
//pub(crate) use self::option::option_generic;

mod string;
pub use self::string::{make_string_name, new_string_type};

mod r#struct;
pub use self::r#struct::new_struct_type;

mod tuple;
pub(crate) use self::tuple::make_tuple_name;
pub use self::tuple::tuple_generic;

mod tuple_struct;
pub use self::tuple_struct::new_tuple_struct_type;

mod unit_struct;
pub use self::unit_struct::new_unit_struct_type;

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

pub(crate) enum Variant {
	Unit(Box<str>),
	Tuple(Box<str>, Box<[TypeId]>),
	Struct(Box<str>, HashMap<Box<str>, usize>, Box<[TypeId]>),
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

pub(crate) struct Type {
	pub name: Arc<str>,
	pub kind: TypeKind,

	pub type_parameters: Arc<[TypeId]>,

	pub field_names: Arc<HashMap<Box<str>, usize>>,
	pub field_types: Arc<[TypeId]>,

	pub variant_names: Arc<HashMap<Box<str>, usize>>,
	pub variants: Arc<[Variant]>,

	pub functions: Arc<HashMap<Box<str>, MemberFunction>>,
}

impl Default for Type {
	fn default() -> Self {
		Self {
			name: "".into(),
			kind: TypeKind::Tuple,
			type_parameters: Arc::new([]),
			field_names: Default::default(),
			field_types: Arc::new([]),
			variant_names: Default::default(),
			variants: Arc::new([]),
			functions: Default::default(),
		}
	}
}

pub enum VariantBuilder {
	Unit(String),
	Tuple(String, Vec<String>),
	Struct(String, HashMap<String, usize>, Vec<String>),
}

pub struct TypeBuilder {
	pub name: String,
	pub kind: TypeKind,

	pub type_parameters: Vec<String>,

	pub field_names: HashMap<String, usize>,
	pub field_types: Vec<String>,

	pub variant_names: HashMap<String, usize>,
	pub variants: Vec<VariantBuilder>,

	pub functions: HashMap<String, MemberFunction>,
}

impl TypeBuilder {
	pub fn new(name: impl Into<String>, kind: TypeKind) -> Self {
		Self {
			name: name.into(),
			kind,
			type_parameters: Default::default(),
			field_names: Default::default(),
			field_types: Default::default(),
			variant_names: Default::default(),
			variants: Default::default(),
			functions: Default::default(),
		}
	}

	pub fn from_generic_type_builder(name: impl Into<String>, generic_type_builder: GenericTypeBuilder) -> Self {
		Self {
			name: name.into(),
			kind: generic_type_builder.kind,
			type_parameters: generic_type_builder.type_parameters,
			field_names: generic_type_builder.field_names,
			field_types: generic_type_builder.field_types,
			variant_names: generic_type_builder.variant_names,
			variants: generic_type_builder.variants,
			functions: generic_type_builder.functions,
		}
	}
}

pub struct GenericTypeBuilder {
	pub kind: TypeKind,
	pub type_parameters: Vec<String>,

	pub field_names: HashMap<String, usize>,
	pub field_types: Vec<String>,

	pub variant_names: HashMap<String, usize>,
	pub variants: Vec<VariantBuilder>,

	pub functions: HashMap<String, MemberFunction>,
}
