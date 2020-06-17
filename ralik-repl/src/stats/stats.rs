use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub(super) struct Stats {
	values: Values,
}

#[derive(Serialize, Clone, Debug)]
struct Values {
	size: usize,
	alignment: usize,
}

impl std::fmt::Display for Stats {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", toml::ser::to_string_pretty(self).unwrap())
	}
}

impl Stats {
	pub fn new() -> Self {
		Self { values: Values::new() }
	}
}

impl Values {
	pub fn new() -> Self {
		Self {
			size: std::mem::size_of::<ralik::Value>(),
			alignment: std::mem::align_of::<ralik::Value>(),
		}
	}
}
