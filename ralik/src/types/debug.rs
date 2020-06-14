use std::collections::hash_map::HashMap;

use super::MemberFunction;

pub(crate) struct FunctionNameListFormatter<'a>(pub(crate) &'a HashMap<String, MemberFunction>);

impl<'a> From<&'a HashMap<String, MemberFunction>> for FunctionNameListFormatter<'a> {
	fn from(map_ref: &'a HashMap<String, MemberFunction>) -> Self {
		Self(map_ref)
	}
}

impl<'a> std::fmt::Debug for FunctionNameListFormatter<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.keys()).finish()
	}
}
