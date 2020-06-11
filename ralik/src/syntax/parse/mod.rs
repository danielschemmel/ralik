use num_bigint::BigInt;
use syn::{braced, bracketed, parenthesized, parse, Ident, LitBool, LitChar, LitInt, LitStr, Token};

use super::ast;

#[cfg(test)]
mod test;

impl parse::Parse for ast::Expression {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let mut prefixes = Vec::new();
		loop {
			let lookahead = input.lookahead1();
			if lookahead.peek(Token![!]) || lookahead.peek(Token![-]) {
				prefixes.push(input.parse::<ast::Prefix>()?);
			} else {
				break;
			}
		}

		let mut expression = ast::Expression::Atomic(input.parse::<ast::AtomicExpression>()?);
		while !prefixes.is_empty() {
			expression = ast::Expression::Prefix(Box::new(expression), prefixes.pop().unwrap())
		}

		while !input.is_empty() {
			let lookahead = input.lookahead1();
			if lookahead.peek(Token![?]) || lookahead.peek(Token![.]) || lookahead.peek(syn::token::Bracket) {
				expression = ast::Expression::Suffix(Box::new(expression), input.parse::<ast::Suffix>()?);
			} else {
				break;
			}
		}

		Ok(expression)
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

impl parse::Parse for ast::Suffix {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let lookahead = input.lookahead1();
		if lookahead.peek(Token![?]) {
			let token = input.parse::<Token![?]>()?;
			Ok(ast::Suffix::Unwrap(token.span))
		} else if lookahead.peek(Token![.]) {
			input.parse::<Token![.]>()?;
			let lookahead = input.lookahead1();
			if lookahead.peek(Ident) {
				let id = input.parse::<Ident>()?;

				let lookahead = input.lookahead1();
				if lookahead.peek(syn::token::Paren) {
					let parenthesized;
					parenthesized!(parenthesized in input);
					let arguments = parse_arguments(&parenthesized)?;
					Ok(ast::Suffix::FunctionCall(id.to_string(), id.span(), arguments, parenthesized.span()))
				} else {
					Ok(ast::Suffix::Field(id.to_string(), id.span()))
				}
			} else if lookahead.peek(LitInt) {
				let tuple_index = input.parse::<LitInt>()?;
				Ok(ast::Suffix::TupleIndex(tuple_index.base10_parse()?, tuple_index.span()))
			} else {
				Err(lookahead.error())
			}
		} else if lookahead.peek(syn::token::Bracket) {
			let bracketed;
			bracketed!(bracketed in input);
			let expression = bracketed.parse::<ast::Expression>()?;
			Ok(ast::Suffix::ArrayIndex(Box::new(expression), bracketed.span()))
		} else {
			Err(lookahead.error())
		}
	}
}

impl parse::Parse for ast::AtomicExpression {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let lookahead = input.lookahead1();
		if lookahead.peek(syn::token::Paren) {
			let parenthesized;
			parenthesized!(parenthesized in input);
			let expression = parenthesized.parse::<ast::Expression>()?;
			Ok(ast::AtomicExpression::Parenthesized(
				Box::new(expression),
				parenthesized.span(),
			))
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
				let (arguments, span) = (parse_arguments(&arguments)?, arguments.span());
				Ok(ast::AtomicExpression::FunctionCall(id.to_string(), arguments, span))
			} else if lookahead.peek(Token![!]) {
				input.parse::<Token![!]>()?;
				let lookahead = input.lookahead1();
				let (arguments, span) = if lookahead.peek(syn::token::Paren) {
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
				Ok(ast::AtomicExpression::MacroCall(id.to_string(), arguments, span))
			} else {
				Err(lookahead.error())
			}
		} else {
			Err(lookahead.error())
		}
	}
}

fn parse_arguments(input: parse::ParseStream) -> parse::Result<ast::Arguments> {
	let mut arguments = Vec::new();
	if !input.is_empty() {
		arguments.push(input.parse::<ast::Expression>()?);
		parse_trailing_arguments_impl(&mut arguments, input)?;
	}
	Ok(ast::Arguments { arguments })
}

fn parse_trailing_arguments_impl(args: &mut Vec<ast::Expression>, input: parse::ParseStream) -> parse::Result<()> {
	while !input.is_empty() {
		input.parse::<Token![,]>()?;
		args.push(input.parse::<ast::Expression>()?);
	}
	Ok(())
}
