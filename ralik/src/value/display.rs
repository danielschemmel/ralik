use std::fmt;

use crate::types::{TypeKind, Variant};

use super::{Data, Value};

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.r#type.kind() {
			TypeKind::Bool => match &self.data {
				Data::Bool(value) => value.fmt(f),
				_ => panic!("Invalid bool representation"),
			},
			TypeKind::Integer => match &self.data {
				Data::Integer(value) => value.fmt(f),
				_ => panic!("Invalid integer representation"),
			},
			TypeKind::Char => match &self.data {
				Data::Char(value) => {
					write!(f, "'")?;
					value.escape_debug().fmt(f)?;
					write!(f, "'")
				}
				_ => panic!("Invalid char representation"),
			},
			TypeKind::String => match &self.data {
				Data::String(value) => {
					write!(f, "'")?;
					value.escape_debug().fmt(f)?;
					write!(f, "'")
				}
				_ => panic!("Invalid char representation"),
			},
			TypeKind::Tuple => match &self.data {
				Data::Empty => write!(f, "()"),
				Data::Array(value) => {
					assert!(value.len() > 0);
					write!(f, "(")?;
					for (i, element) in value.iter().enumerate() {
						if i > 0 {
							write!(f, ", {}", element)?;
						} else {
							write!(f, "{}", element)?;
						}
					}
					if value.len() == 1 {
						write!(f, ", )")
					} else {
						write!(f, ")")
					}
				}
				_ => panic!("Invalid tuple representation"),
			},
			TypeKind::UnitStruct => match &self.data {
				Data::Empty => write!(f, "{}", self.r#type.name()),
				_ => panic!("Invalid unit struct representation"),
			},
			TypeKind::TupleStruct => {
				write!(f, "{}", self.r#type.name())?;
				match &self.data {
					Data::Empty => write!(f, "()"),
					Data::Array(value) => {
						assert!(value.len() > 0);
						write!(f, "(")?;
						for (i, element) in value.iter().enumerate() {
							if i > 0 {
								write!(f, ", {}", element)?;
							} else {
								write!(f, "{}", element)?;
							}
						}
						write!(f, ")")
					}
					_ => panic!("Invalid tuple struct representation"),
				}
			}
			TypeKind::Struct => {
				write!(f, "{} {{", self.r#type.name())?;
				match &self.data {
					Data::Empty => {}
					Data::Array(value) => {
						for (i, (name, id)) in self.r#type.fields().0.iter().enumerate() {
							if i > 0 {
								write!(f, ", {}: {}", name, value[*id])?;
							} else {
								write!(f, " {}: {}", name, value[*id])?;
							}
						}
					}
					_ => panic!("Invalid struct representation"),
				}
				write!(f, " }}")
			}
			TypeKind::Enum => {
				write!(f, "{}::", self.r#type.name())?;
				match &self.data {
					Data::UnitVariant(id) => match &self.r#type.variants().1[*id] {
						Variant::Unit(name) => write!(f, "{}", name),
						Variant::Tuple(name, _field_types) => write!(f, "{}()", name),
						Variant::Struct(name, _field_names, _field_types) => write!(f, "{} {{ }}", name),
					},
					Data::Variant(id, value) => match &self.r#type.variants().1[*id] {
						Variant::Unit(name) => write!(f, "{}", name),
						Variant::Tuple(name, _field_types) => {
							write!(f, "{}(", name)?;
							for (i, element) in value.iter().enumerate() {
								if i > 0 {
									write!(f, ", {}", element)?;
								} else {
									write!(f, "{}", element)?;
								}
							}
							write!(f, ")")
						}
						Variant::Struct(name, field_names, _field_types) => {
							write!(f, "{} {{ ", name)?;
							for (i, (name, id)) in field_names.iter().enumerate() {
								if i > 0 {
									write!(f, ", {}: {}", name, value[*id])?;
								} else {
									write!(f, " {}: {}", name, value[*id])?;
								}
							}
							write!(f, " }}")
						}
					},
					_ => panic!("Invalid enum representation"),
				}
			}
			TypeKind::Array => match &self.data {
				Data::Empty => write!(f, "[]"),
				Data::Array(value) => {
					write!(f, "[")?;
					for (i, element) in value.iter().enumerate() {
						if i > 0 {
							write!(f, ", {}", element)?;
						} else {
							write!(f, "{}", element)?;
						}
					}
					write!(f, "]")
				}
				_ => panic!("Invalid array representation"),
			},
		}
	}
}
