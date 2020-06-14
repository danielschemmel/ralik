use num_bigint::BigInt;

use std::fmt;

impl fmt::Debug for super::AtomicExpression {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use super::AtomicExpression::*;
		match self {
			Unit(span) => f.debug_tuple("Unit").field(span).finish(),
			Parenthesized(expression, span) => f.debug_tuple("Parenthesized").field(expression).field(span).finish(),
			Tuple(expressions, span) => f.debug_tuple("Tuple").field(expressions).field(span).finish(),
			Array(expressions, span) => f.debug_tuple("Array").field(expressions).field(span).finish(),
			LitBool(value, span) => f.debug_tuple("LitBool").field(&value).field(span).finish(),
			LitInt(value, span) => f
				.debug_tuple("LitInt")
				.field(&IntegerFormatter(value))
				.field(span)
				.finish(),
			LitByte(value, span) => f.debug_tuple("LitByte").field(&value).field(span).finish(),
			LitByteStr(value, span) => f.debug_tuple("LitByteStr").field(&value).field(span).finish(),
			LitChar(value, span) => f.debug_tuple("LitChar").field(&value).field(span).finish(),
			LitStr(value, span) => f.debug_tuple("LitStr").field(&value).field(span).finish(),
			Dollar(span) => f.debug_tuple("Dollar").field(span).finish(),
			FunctionCall(name, name_span, arguments, arguments_span) => f
				.debug_tuple("FunctionCall")
				.field(&name)
				.field(&name_span)
				.field(&arguments)
				.field(&arguments_span)
				.finish(),
			MacroCall(name, name_span, arguments, arguments_span) => f
				.debug_tuple("MacroCall")
				.field(&name)
				.field(&name_span)
				.field(&arguments)
				.field(&arguments_span)
				.finish(),
		}
	}
}

struct IntegerFormatter<'a>(&'a BigInt);

impl<'a> fmt::Debug for IntegerFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.0, f)
	}
}
