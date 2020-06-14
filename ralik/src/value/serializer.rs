use my_serde::ser;
use thiserror::Error;

use super::Value;
use crate::Context;

impl Value {
	pub fn from_serde<T: ser::Serialize>(context: &Context, value: T) -> Self {
		let serializer = Serializer { context };
		value.serialize(serializer).unwrap()
	}
}

#[derive(Error, Debug)]
enum Error {
	#[error("Floating point numbers are (currently?) not supported by RALIK")]
	Float,

	#[error("Empty arrays are (currently?) not supported by RALIK")]
	EmptyArray,

	#[error(transparent)]
	ValueCreationError(crate::error::ValueCreationError),

	#[error("Custom Error: {0}")]
	Custom(String),
}

impl<T: Into<crate::error::ValueCreationError>> From<T> for Error {
	fn from(value: T) -> Self {
		Error::ValueCreationError(value.into())
	}
}

impl ser::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		Error::Custom(msg.to_string())
	}
}

struct Serializer<'a> {
	context: &'a Context,
}

impl<'a> ser::Serializer for Serializer<'a> {
	type Ok = Value;
	type Error = Error;
	type SerializeSeq = SerializeSeq<'a>;
	type SerializeTuple = SerializeTuple<'a>;
	type SerializeTupleStruct = SerializeTupleStruct;
	type SerializeTupleVariant = SerializeTupleVariant;
	type SerializeMap = SerializeMap;
	type SerializeStruct = SerializeStruct;
	type SerializeStructVariant = SerializeStructVariant;

	fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_bool(self.context, value)?)
	}

	fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_integer(self.context, value)?)
	}

	fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
		Err(Error::Float)
	}

	fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
		Err(Error::Float)
	}

	fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_char(self.context, value)?)
	}

	fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_string(self.context, value)?)
	}

	fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
		let integer_type = self
			.context
			.get_integer_type()
			.map_err(crate::error::IntegerCreationError::InvalidType)?;
		let values = value
			.iter()
			.map(|byte| Value::new_integer(self.context, *byte))
			.collect::<Result<Vec<Value>, crate::error::IntegerCreationError>>()?;
		Ok(Value::new_array(self.context, &integer_type, values)?)
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}

	fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_unit(self.context)?)
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}

	fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Ok(SerializeSeq(
			self.context,
			if let Some(len) = len {
				Vec::with_capacity(len)
			} else {
				Vec::new()
			},
		))
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Ok(SerializeTuple(self.context, Vec::with_capacity(len)))
	}

	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		unimplemented!()
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		unimplemented!()
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		unimplemented!()
	}

	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		unimplemented!()
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		unimplemented!()
	}
}

struct SerializeSeq<'a>(&'a Context, Vec<Value>);

impl<'a> ser::SerializeSeq for SerializeSeq<'a> {
	type Ok = Value;
	type Error = Error;

	fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.1.push(Value::from_serde(self.0, value));
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		if self.1.is_empty() {
			Err(Error::EmptyArray)
		} else {
			Ok(Value::new_array(self.0, &self.1[0].get_type().clone(), self.1)?)
		}
	}
}

struct SerializeTuple<'a>(&'a Context, Vec<Value>);

impl<'a> ser::SerializeTuple for SerializeTuple<'a> {
	type Ok = Value;
	type Error = Error;

	fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.1.push(Value::from_serde(self.0, value));
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Value::new_tuple(self.0, self.1)?)
	}
}

struct SerializeTupleStruct;

impl ser::SerializeTupleStruct for SerializeTupleStruct {
	type Ok = Value;
	type Error = Error;

	fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}
}

struct SerializeTupleVariant;

impl ser::SerializeTupleVariant for SerializeTupleVariant {
	type Ok = Value;
	type Error = Error;

	fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}
}

struct SerializeMap;

impl ser::SerializeMap for SerializeMap {
	type Ok = Value;
	type Error = Error;

	fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}
}

struct SerializeStruct;

impl ser::SerializeStruct for SerializeStruct {
	type Ok = Value;
	type Error = Error;

	fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}
}

struct SerializeStructVariant;

impl ser::SerializeStructVariant for SerializeStructVariant {
	type Ok = Value;
	type Error = Error;

	fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!()
	}
}
