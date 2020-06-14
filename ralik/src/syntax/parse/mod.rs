use num_bigint::BigInt;
use syn::parse::Lookahead1;
use syn::{braced, bracketed, parenthesized, parse, Ident, LitBool, LitChar, LitInt, LitStr, Token};

use super::ast;

#[cfg(test)]
mod test;

impl parse::Parse for ast::Expression {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let (expression, lookahead) = parse_expression(input)?;
		if input.is_empty() {
			Ok(expression)
		} else {
			Err(lookahead.error())
		}
	}
}

fn parse_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	parse_lazy_or_expression(input)
}

fn parse_lazy_or_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_lazy_and_expression(input)?;

	// TODO: Some tokens have a member `spans: [Span; 2]`, which should really be joined once span joining is available.
	loop {
		if lookahead.peek(Token![||]) {
			let token = input.parse::<Token![||]>()?;
			let (rhs_expression, rhs_lookahead) = parse_lazy_and_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::LazyOr(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_lazy_and_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_cmp_expression(input)?;

	// TODO: Some tokens have a member `spans: [Span; 2]`, which should really be joined once span joining is available.
	loop {
		if lookahead.peek(Token![&&]) {
			let token = input.parse::<Token![&&]>()?;
			let (rhs_expression, rhs_lookahead) = parse_cmp_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::LazyAnd(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_cmp_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (expression, lookahead) = parse_bit_or_expression(input)?;

	// TODO: Some tokens have a member `spans: [Span; 2]`, which should really be joined once span joining is available.
	if lookahead.peek(Token![==]) {
		let token = input.parse::<Token![==]>()?;
		let (rhs_expression, rhs_lookahead) = parse_bit_or_expression(input)?;
		Ok((
			ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Equal(token.spans[0]),
			),
			rhs_lookahead,
		))
	} else if lookahead.peek(Token![!=]) {
		let token = input.parse::<Token![!=]>()?;
		let (rhs_expression, rhs_lookahead) = parse_bit_or_expression(input)?;
		Ok((
			ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::NotEqual(token.spans[0]),
			),
			rhs_lookahead,
		))
	} else if lookahead.peek(Token![<=]) {
		let token = input.parse::<Token![<=]>()?;
		let (rhs_expression, rhs_lookahead) = parse_bit_or_expression(input)?;
		Ok((
			ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::LessOrEqual(token.spans[0]),
			),
			rhs_lookahead,
		))
	} else if lookahead.peek(Token![>=]) {
		let token = input.parse::<Token![>=]>()?;
		let (rhs_expression, rhs_lookahead) = parse_bit_or_expression(input)?;
		Ok((
			ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::GreaterOrEqual(token.spans[0]),
			),
			rhs_lookahead,
		))
	} else if lookahead.peek(Token![<]) && !input.peek(Token![<<]) && !input.peek(Token![<=]) {
		let token = input.parse::<Token![<]>()?;
		let (rhs_expression, rhs_lookahead) = parse_bit_or_expression(input)?;
		Ok((
			ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Less(token.spans[0]),
			),
			rhs_lookahead,
		))
	} else if lookahead.peek(Token![>]) && !input.peek(Token![>>]) && !input.peek(Token![>=]) {
		let token = input.parse::<Token![>]>()?;
		let (rhs_expression, rhs_lookahead) = parse_bit_or_expression(input)?;
		Ok((
			ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Greater(token.spans[0]),
			),
			rhs_lookahead,
		))
	} else {
		Ok((expression, lookahead))
	}
}

fn parse_bit_or_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_bit_xor_expression(input)?;

	loop {
		if lookahead.peek(Token![|]) && !input.peek(Token![||]) {
			let token = input.parse::<Token![|]>()?;
			let (rhs_expression, rhs_lookahead) = parse_bit_xor_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::BitOr(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_bit_xor_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_bit_and_expression(input)?;

	loop {
		if lookahead.peek(Token![^]) {
			let token = input.parse::<Token![^]>()?;
			let (rhs_expression, rhs_lookahead) = parse_bit_and_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::BitXor(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_bit_and_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_shift_expression(input)?;

	loop {
		if lookahead.peek(Token![&]) && !input.peek(Token![&&]) {
			let token = input.parse::<Token![&]>()?;
			let (rhs_expression, rhs_lookahead) = parse_shift_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::BitAnd(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_shift_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_additive_expression(input)?;

	// TODO: Some tokens have a member `spans: [Span; 2]`, which should really be joined once span joining is available.
	loop {
		if lookahead.peek(Token![<<]) {
			let token = input.parse::<Token![<<]>()?;
			let (rhs_expression, rhs_lookahead) = parse_additive_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Shl(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else if lookahead.peek(Token![>>]) {
			let token = input.parse::<Token![>>]>()?;
			let (rhs_expression, rhs_lookahead) = parse_additive_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Shr(token.spans[0]),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_additive_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_multiplicative_expression(input)?;

	loop {
		if lookahead.peek(Token![+]) {
			let token = input.parse::<Token![+]>()?;
			let (rhs_expression, rhs_lookahead) = parse_multiplicative_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Add(token.span),
			);
			lookahead = rhs_lookahead;
		} else if lookahead.peek(Token![-]) {
			let token = input.parse::<Token![-]>()?;
			let (rhs_expression, rhs_lookahead) = parse_multiplicative_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Sub(token.span),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_multiplicative_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let (mut expression, mut lookahead) = parse_prefix_expression(input)?;

	loop {
		if lookahead.peek(Token![*]) {
			let token = input.parse::<Token![*]>()?;
			let (rhs_expression, rhs_lookahead) = parse_prefix_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Mul(token.span),
			);
			lookahead = rhs_lookahead;
		} else if lookahead.peek(Token![/]) {
			let token = input.parse::<Token![/]>()?;
			let (rhs_expression, rhs_lookahead) = parse_prefix_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Div(token.span),
			);
			lookahead = rhs_lookahead;
		} else if lookahead.peek(Token![%]) {
			let token = input.parse::<Token![%]>()?;
			let (rhs_expression, rhs_lookahead) = parse_prefix_expression(input)?;
			expression = ast::Expression::Binary(
				Box::new(expression),
				Box::new(rhs_expression),
				ast::BinaryOperator::Rem(token.span),
			);
			lookahead = rhs_lookahead;
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_prefix_expression(input: parse::ParseStream) -> parse::Result<(ast::Expression, Lookahead1)> {
	let lookahead = input.lookahead1();
	if lookahead.peek(Token![!]) {
		let token = input.parse::<Token![!]>()?;
		let prefix = ast::Prefix::Not(token.span);
		let (expression, lookahead) = parse_prefix_expression(input)?;
		Ok((ast::Expression::Prefix(Box::new(expression), prefix), lookahead))
	} else if lookahead.peek(Token![-]) {
		let token = input.parse::<Token![-]>()?;
		let prefix = ast::Prefix::Minus(token.span);
		let (expression, lookahead) = parse_prefix_expression(input)?;
		Ok((ast::Expression::Prefix(Box::new(expression), prefix), lookahead))
	} else {
		parse_suffix_expression(input, lookahead)
	}
}

fn parse_suffix_expression<'a>(
	input: parse::ParseStream<'a>,
	lookahead: Lookahead1,
) -> parse::Result<(ast::Expression, Lookahead1<'a>)> {
	let mut expression = ast::Expression::Atomic(parse_atomic_expression(input, lookahead)?);

	loop {
		let lookahead = input.lookahead1();
		if lookahead.peek(Token![?]) {
			let token = input.parse::<Token![?]>()?;
			expression = ast::Expression::Suffix(Box::new(expression), ast::Suffix::Unwrap(token.span))
		} else if lookahead.peek(Token![.]) && !input.peek(Token![..]) {
			input.parse::<Token![.]>()?;
			expression = parse_field_expression(input, expression)?;
		} else if lookahead.peek(syn::token::Bracket) {
			let bracketed;
			bracketed!(bracketed in input);
			let index = bracketed.parse::<ast::Expression>()?;
			expression = ast::Expression::Suffix(
				Box::new(expression),
				ast::Suffix::ArrayIndex(Box::new(index), bracketed.span()),
			)
		} else {
			return Ok((expression, lookahead));
		}
	}
}

fn parse_field_expression(input: parse::ParseStream, expression: ast::Expression) -> parse::Result<ast::Expression> {
	let lookahead = input.lookahead1();
	if lookahead.peek(Ident) {
		let id = input.parse::<Ident>()?;

		let lookahead = input.lookahead1();
		if lookahead.peek(syn::token::Paren) {
			let parenthesized;
			parenthesized!(parenthesized in input);
			let arguments = parse_arguments(&parenthesized)?;
			Ok(ast::Expression::Suffix(
				Box::new(expression),
				ast::Suffix::FunctionCall(id.to_string(), id.span(), arguments, parenthesized.span()),
			))
		} else {
			Ok(ast::Expression::Suffix(
				Box::new(expression),
				ast::Suffix::Field(id.to_string(), id.span()),
			))
		}
	} else if lookahead.peek(LitInt) {
		let tuple_index = input.parse::<LitInt>()?;
		Ok(ast::Expression::Suffix(
			Box::new(expression),
			ast::Suffix::TupleIndex(tuple_index.base10_parse()?, tuple_index.span()),
		))
	} else {
		Err(lookahead.error())
	}
}

fn parse_atomic_expression(input: parse::ParseStream, lookahead: Lookahead1) -> parse::Result<ast::AtomicExpression> {
	if lookahead.peek(syn::token::Paren) {
		let parenthesized;
		parenthesized!(parenthesized in input);
		if parenthesized.is_empty() {
			return Ok(ast::AtomicExpression::Unit(parenthesized.span()));
		}

		let (expression, lookahead) = parse_expression(&parenthesized)?;

		if parenthesized.is_empty() {
			Ok(ast::AtomicExpression::Parenthesized(
				Box::new(expression),
				parenthesized.span(),
			))
		} else if lookahead.peek(Token![,]) {
			parenthesized.parse::<Token![,]>()?;
			let mut tuple = Vec::new();
			tuple.push(expression);

			while !parenthesized.is_empty() {
				let (expression, lookahead) = parse_expression(&parenthesized)?;
				tuple.push(expression);
				if parenthesized.is_empty() {
					break;
				}
				if lookahead.peek(Token![,]) {
					parenthesized.parse::<Token![,]>()?;
				} else {
					return Err(lookahead.error());
				}
			}
			Ok(ast::AtomicExpression::Tuple(tuple, parenthesized.span()))
		} else {
			Err(lookahead.error())
		}
	} else if lookahead.peek(syn::token::Bracket) {
		let bracketed;
		bracketed!(bracketed in input);
		let mut array = Vec::new();
		while !bracketed.is_empty() {
			let (expression, lookahead) = parse_expression(&bracketed)?;
			array.push(expression);
			if bracketed.is_empty() {
				break;
			}
			if lookahead.peek(Token![,]) {
				bracketed.parse::<Token![,]>()?;
			} else {
				return Err(lookahead.error());
			}
		}
		Ok(ast::AtomicExpression::Array(array, bracketed.span()))
	} else if lookahead.peek(LitBool) {
		let lit_bool = input.parse::<LitBool>()?;
		Ok(ast::AtomicExpression::LitBool(lit_bool.value, lit_bool.span))
	} else if lookahead.peek(LitChar) {
		let lit_char = input.parse::<LitChar>()?;
		Ok(ast::AtomicExpression::LitChar(lit_char.value(), lit_char.span()))
	} else if lookahead.peek(LitInt) {
		let lit_int = input.parse::<LitInt>()?;
		if lit_int.suffix() != "" {
			return Err(syn::Error::new(
				lit_int.span(),
				"Integer suffix is not supported in [build-info] yet",
			));
		}
		Ok(ast::AtomicExpression::LitInt(
			lit_int.base10_parse::<BigInt>()?,
			lit_int.span(),
		))
	} else if lookahead.peek(LitStr) {
		let lit_str = input.parse::<LitStr>()?;
		Ok(ast::AtomicExpression::LitStr(lit_str.value(), lit_str.span()))
	} else if lookahead.peek(Token![$]) {
		let token = input.parse::<Token![$]>()?;
		Ok(ast::AtomicExpression::Dollar(token.spans[0]))
	} else if lookahead.peek(Ident) {
		let id = input.parse::<Ident>()?;

		let lookahead = input.lookahead1();
		if lookahead.peek(syn::token::Paren) {
			let arguments;
			parenthesized!(arguments in input);
			let (arguments, arguments_span) = (parse_arguments(&arguments)?, arguments.span());
			Ok(ast::AtomicExpression::FunctionCall(
				id.to_string(),
				id.span(),
				arguments,
				arguments_span,
			))
		} else if lookahead.peek(Token![!]) {
			input.parse::<Token![!]>()?;
			let lookahead = input.lookahead1();
			let (arguments, arguments_span) = if lookahead.peek(syn::token::Paren) {
				let arguments;
				parenthesized!(arguments in input);
				(parse_arguments(&arguments)?, arguments.span())
			} else if lookahead.peek(syn::token::Brace) {
				let arguments;
				braced!(arguments in input);
				(parse_arguments(&arguments)?, arguments.span())
			} else if lookahead.peek(syn::token::Bracket) {
				let arguments;
				bracketed!(arguments in input);
				(parse_arguments(&arguments)?, arguments.span())
			} else {
				return Err(lookahead.error());
			};
			Ok(ast::AtomicExpression::MacroCall(
				id.to_string(),
				id.span(),
				arguments,
				arguments_span,
			))
		} else {
			Err(lookahead.error())
		}
	} else {
		Err(lookahead.error())
	}
}

impl parse::Parse for ast::Prefix {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let lookahead = input.lookahead1();
		if lookahead.peek(Token![!]) {
			let token = input.parse::<Token![!]>()?;
			Ok(ast::Prefix::Not(token.span))
		} else if lookahead.peek(Token![-]) {
			let token = input.parse::<Token![-]>()?;
			Ok(ast::Prefix::Minus(token.span))
		} else {
			Err(lookahead.error())
		}
	}
}

fn parse_arguments(input: parse::ParseStream) -> parse::Result<ast::Arguments> {
	let mut arguments = Vec::new();
	if !input.is_empty() {
		let (expression, lookahead) = parse_expression(input)?;
		arguments.push(expression);
		parse_trailing_arguments_impl(&mut arguments, input, lookahead)?;
	}

	debug_assert!(input.is_empty());
	Ok(ast::Arguments { arguments })
}

fn parse_trailing_arguments_impl<'a>(
	arguments: &mut Vec<ast::Expression>,
	input: parse::ParseStream<'a>,
	mut lookahead: Lookahead1<'a>,
) -> parse::Result<()> {
	while !input.is_empty() {
		if lookahead.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			arguments.push(input.parse::<ast::Expression>()?);
			lookahead = input.lookahead1();
		} else {
			return Err(lookahead.error());
		}
	}
	Ok(())
}
