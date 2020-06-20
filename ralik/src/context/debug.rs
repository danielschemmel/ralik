use std::collections::HashMap;

use super::{Context, Function, Macro};

impl std::fmt::Debug for Context {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unimplemented!()
	}
}

struct FunctionNameListFormatter<'a> {
	functions: &'a HashMap<String, Function>,
}
impl<'a> std::fmt::Debug for FunctionNameListFormatter<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.functions.keys()).finish()
	}
}

struct MacroNameListFormatter<'a> {
	macros: &'a HashMap<String, Macro>,
}
impl<'a> std::fmt::Debug for MacroNameListFormatter<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.macros.keys()).finish()
	}
}
