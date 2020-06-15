use std::fmt;

use super::{Data, Value};

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.data {
			Data::Unit => write!(f, "()"),
			Data::Bool(value) => value.fmt(f),
			Data::Integer(value) => value.fmt(f),
			Data::Char(value) => {
				write!(f, "'")?;
				value.escape_debug().fmt(f)?;
				write!(f, "'")
			}
			Data::String(value) => {
				write!(f, "\"")?;
				value.escape_debug().fmt(f)?;
				write!(f, "\"")
			}
			Data::Tuple(value) => {
				assert!(value.len() > 0);
				if value.len() == 1 {
					write!(f, "({}, )", value[0])
				} else {
					write!(f, "(")?;
					for (i, element) in value.iter().enumerate() {
						if i > 0 {
							write!(f, ", ")?;
						}
						element.fmt(f)?;
					}
					write!(f, ")")
				}
			}
			Data::Struct(value) => {
				write!(f, "{} {{", self.r#type.name())?;
				for (i, (field_name, field_value)) in value.iter().enumerate() {
					if i > 0 {
						write!(f, ",")?;
					}
					write!(f, " {}: {}", field_name, field_value)?;
				}
				write!(f, " }}")
			}
			Data::Array(value) => {
				write!(f, "[")?;
				for (i, element) in value.iter().enumerate() {
					if i > 0 {
						write!(f, ", ")?;
					}
					write!(f, "{}", element)?;
				}
				write!(f, "]")
			}
		}
	}
}
