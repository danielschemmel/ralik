use num::BigInt;
use proc_macro2::Span;

mod debug;

#[derive(Clone, Debug)]
pub enum Expression {
	Atomic(AtomicExpression),
	Suffix(Box<Expression>, Suffix),
	Prefix(Box<Expression>, Prefix),
	Binary(Box<Expression>, Box<Expression>, BinaryOperator),
	Block(Block),
	If(Span, Box<Expression>, Block),
	Else(Box<Expression>, Span, Block),
	While(Span, Box<Expression>, Block),
	Loop(Option<(String, Span)>, Span, Block),
}

impl Expression {
	pub fn span(&self) -> Span {
		match self {
			Expression::Atomic(expr) => expr.span(),
			Expression::Suffix(_, suffix) => suffix.span(),
			Expression::Prefix(_, prefix) => prefix.span(),
			Expression::Binary(_lhs, _rhs, op) => op.span(),
			Expression::Block(block) => block.span(),
			Expression::If(if_span, _condition, _body) => *if_span,
			Expression::Else(_lhs, else_span, _body) => *	else_span,
			Expression::While(while_span, _condition, _body) => *while_span,
			Expression::Loop(_label, loop_span, _body) => *loop_span,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Block {
	pub statements: Vec<Statement>,
	pub expression: Option<Box<Expression>>,
	pub span: Span,
}

impl Block {
	pub fn span(&self) -> Span {
		self.span
	}
}

#[derive(Clone, Debug)]
pub enum Statement {
	Expression(Expression),
	Let(Span, bool, Pattern, Option<Expression>),
}

impl Statement {
	pub fn span(&self) -> Span {
		match self {
			Statement::Expression(expression) => expression.span(),
			Statement::Let(let_span, _is_mut, _pattern, _assignment) => *let_span,
		}
	}
}

#[derive(Clone, Debug)]
pub enum Pattern {
	Identifier(String, Span),
}

impl Pattern {
	pub fn span(&self) -> Span {
		match self {
			Pattern::Identifier(_id, span ) => *span,
		}
	}
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

impl BinaryOperator {
	pub fn span(&self) -> Span {
		match self {
			BinaryOperator::Mul(span)
			| BinaryOperator::Div(span)
			| BinaryOperator::Rem(span)
			| BinaryOperator::Add(span)
			| BinaryOperator::Sub(span)
			| BinaryOperator::Shl(span)
			| BinaryOperator::Shr(span)
			| BinaryOperator::BitAnd(span)
			| BinaryOperator::BitXor(span)
			| BinaryOperator::BitOr(span)
			| BinaryOperator::Equal(span)
			| BinaryOperator::NotEqual(span)
			| BinaryOperator::Less(span)
			| BinaryOperator::LessOrEqual(span)
			| BinaryOperator::Greater(span)
			| BinaryOperator::GreaterOrEqual(span)
			| BinaryOperator::LazyAnd(span)
			| BinaryOperator::LazyOr(span) => *span,
		}
	}
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

impl AtomicExpression {
	pub fn span(&self) -> Span {
		match self {
			AtomicExpression::Unit(span)
			| AtomicExpression::Parenthesized(_, span)
			| AtomicExpression::Tuple(_, span)
			| AtomicExpression::Array(_, span)
			| AtomicExpression::LitBool(_, span)
			| AtomicExpression::LitInt(_, span)
			| AtomicExpression::LitByte(_, span)
			| AtomicExpression::LitByteStr(_, span)
			| AtomicExpression::LitChar(_, span)
			| AtomicExpression::LitStr(_, span)
			| AtomicExpression::Dollar(span) => *span,
			AtomicExpression::FunctionCall(_, name_span, _, _arguments_span)
			| AtomicExpression::MacroCall(_, name_span, _, _arguments_span) => *name_span,
		}
	}
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

impl Prefix {
	pub fn span(&self) -> Span {
		match self {
			Prefix::Not(span) | Prefix::Minus(span) => *span,
		}
	}
}

#[derive(Clone, Debug)]
pub enum Suffix {
	Unwrap(Span),
	Field(String, Span),
	TupleIndex(u32, Span),
	ArrayIndex(Box<Expression>, Span),
	FunctionCall(String, Span, Arguments, Span),
}

impl Suffix {
	pub fn span(&self) -> Span {
		match self {
			Suffix::Unwrap(span) | Suffix::Field(_, span) | Suffix::TupleIndex(_, span) | Suffix::ArrayIndex(_, span) => {
				*span
			}
			Suffix::FunctionCall(_, name_span, _, _arguments_span) => *name_span,
		}
	}
}
