use num_bigint::BigInt;
use proc_macro2::Span;

mod debug;
mod eq;

#[derive(Clone, Debug)]
pub enum Expression {
	Atomic(AtomicExpression),
	Suffix(Box<Expression>, Suffix),
	Prefix(Box<Expression>, Prefix),
}

#[derive(Clone)]
pub enum AtomicExpression {
	Parenthesized(Box<Expression>, Span),
	LitBool(bool, Span),
	LitInt(BigInt, Span),
	LitChar(char, Span),
	LitStr(String, Span),
	Dollar(Span),
	FunctionCall(String, Arguments, Span),
	MacroCall(String, Arguments, Span),
}

#[derive(Clone, Debug)]
pub struct Arguments {
	pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Prefix {
	Not(Span),
	Minus(Span),
}

#[derive(Clone, Debug)]
pub enum Suffix {
	Unwrap(Span),
	Field(String, Span),
	TupleIndex(u32, Span),
	ArrayIndex(Box<Expression>, Span),
	FunctionCall(String, Span, Arguments, Span),
}
