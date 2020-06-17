use build_info::semver::Version;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub(super) struct Stats {
	ralik_crate: RalikCrate,
	ralik_repl_crate: RalikReplCrate,
	values: Values,
}

#[derive(Serialize, Clone, Debug)]
struct RalikCrate {
	version: Version,
}

#[derive(Serialize, Clone, Debug)]
struct RalikReplCrate {
	version: Version,
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
		Self {
			ralik_crate: RalikCrate::new(),
			ralik_repl_crate: RalikReplCrate::new(),
			values: Values::new(),
		}
	}
}

impl RalikCrate {
	pub fn new() -> Self {
		let ralik = crate::build_info()
			.crate_info
			.dependencies
			.iter()
			.filter(|dependency| dependency.name == "ralik")
			.nth(0)
			.unwrap();

		Self {
			version: ralik.version.clone(),
		}
	}
}

impl RalikReplCrate {
	pub fn new() -> Self {
		Self {
			version: crate::build_info().crate_info.version.clone(),
		}
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
