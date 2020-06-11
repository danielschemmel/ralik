macro_rules! op {
	($name:ident, $literal:literal) => {
		pub const $name: &str = concat!("[op]::", $literal);
	};
}

op!(PREFIX_NOT, "PrefixNot");
op!(PREFIX_MINUS, "PrefixMinus");
op!(UNWRAP, "Unwrap");
op!(ARRAY_INDEX, "ArrayIndex");
