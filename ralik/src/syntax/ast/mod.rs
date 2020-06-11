use num_bigint::BigInt;
use proc_macro2::Span;

mod debug;
mod eq;

#[derive(Clone, Debug)]
pub struct Expression {
	pub atom: AtomicExpression,
	pub suffixes: Vec<Suffix>,
}

#[derive(Clone)]
pub enum AtomicExpression {
	Dollar(Span),
	LitBool(bool, Span),
	LitInt(BigInt, Span),
	LitChar(char, Span),
	LitStr(String, Span),
	Parenthesized(Box<Expression>, Span),
	FunctionCall(String, Arguments, Span),
	MacroCall(String, Arguments, Span),
}

#[derive(Clone, Debug)]
pub struct Arguments {
	pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum Suffix {
	Unwrap,
	Field(String),
	TupleIndex(u32),
	ArrayIndex(Box<Expression>),
	FunctionCall(String, Arguments),
}
