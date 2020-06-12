macro_rules! op {
	($name:ident, $literal:literal) => {
		pub(crate) const $name: &str = concat!("[op]::", $literal);
	};
}

op!(INDEX, "Index");
op!(UNWRAP, "Unwrap");
op!(NOT, "Not");
op!(NEGATE, "Neg");
op!(MUL, "Mul");
op!(DIV, "Div");
op!(REM, "Rem");
op!(ADD, "Add");
op!(SUB, "Sub");
op!(SHL, "Shl");
op!(SHR, "Shr");
op!(BIT_AND, "BitAnd");
op!(BIT_OR, "BitOr");
op!(BIT_XOR, "BitXor");
op!(EQUAL, "Equal");
op!(NOT_EQUAL, "NotEqual");
op!(LESS, "Less");
op!(LESS_OR_EQUAL, "LessOrEqual");
op!(GREATER, "Greater");
op!(GREATER_OR_EQUAL, "GreaterOrEqual");
