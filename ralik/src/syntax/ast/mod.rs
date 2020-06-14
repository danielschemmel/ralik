use num_bigint::BigInt;
use proc_macro2::Span;

mod debug;

#[derive(Clone, Debug)]
pub enum Expression {
	Atomic(AtomicExpression),
	Suffix(Box<Expression>, Suffix),
	Prefix(Box<Expression>, Prefix),
	Binary(Box<Expression>, Box<Expression>, BinaryOperator),
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
	Mul(Span),
	Div(Span),
	Rem(Span),
	Add(Span),
	Sub(Span),
	Shl(Span),
	Shr(Span),
	BitAnd(Span),
	BitXor(Span),
	BitOr(Span),
	Equal(Span),
	NotEqual(Span),
	Less(Span),
	LessOrEqual(Span),
	Greater(Span),
	GreaterOrEqual(Span),
	LazyAnd(Span),
	LazyOr(Span),
}

#[derive(Clone)]
pub enum AtomicExpression {
	Unit(Span),
	Parenthesized(Box<Expression>, Span),
	Tuple(Vec<Expression>, Span),
	Array(Vec<Expression>, Span),
	LitBool(bool, Span),
	LitInt(BigInt, Span),
	LitByte(u8, Span),
	LitByteStr(Vec<u8>, Span),
	LitChar(char, Span),
	LitStr(String, Span),
	Dollar(Span),
	FunctionCall(String, Span, Arguments, Span),
	MacroCall(String, Span, Arguments, Span),
}

#[derive(Clone, Debug)]
pub struct Arguments {
	pub arguments: Vec<Expression>,
}

#[derive(Copy, Clone, Debug)]
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
