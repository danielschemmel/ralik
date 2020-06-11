use num_bigint::BigInt;
use syn::{braced, bracketed, parenthesized, parse, Ident, LitBool, LitChar, LitInt, LitStr, Token};

use super::ast;


fn parse_arguments(input: parse::ParseStream) -> parse::Result<ast::Arguments> {
	let mut arguments = Vec::new();
	if !input.is_empty() {
		arguments.push(input.parse::<ast::Expression>()?);
		parse_trailing_arguments_impl(&mut arguments, input)?;
	}
	Ok(ast::Arguments { arguments })
}

/*
fn parse_trailing_arguments(input: parse::ParseStream) -> parse::Result<Vec<Expression>> {
	let mut args = Vec::new();
	parse_trailing_arguments_impl(&mut args, input)?;
	Ok(args)
}
*/

fn parse_trailing_arguments_impl(args: &mut Vec<ast::Expression>, input: parse::ParseStream) -> parse::Result<()> {
	while !input.is_empty() {
		input.parse::<Token![,]>()?;
		args.push(input.parse::<ast::Expression>()?);
	}
	Ok(())
}

impl parse::Parse for ast::AtomicExpression {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let lookahead = input.lookahead1();
		if lookahead.peek(Token![$]) {
			let token = input.parse::<Token![$]>()?;
			Ok(ast::AtomicExpression::Dollar(token.spans[0]))
		} else if lookahead.peek(syn::token::Paren) {
			let expr;
			parenthesized!(expr in input);
			Ok(ast::AtomicExpression::Parenthesized(
				Box::new(expr.parse::<ast::Expression>()?),
				expr.span(),
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

impl parse::Parse for ast::Expression {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let atom = input.parse::<ast::AtomicExpression>()?;

		let mut suffixes = Vec::new();
		while !input.is_empty() {
			let lookahead = input.lookahead1();
			if lookahead.peek(Token![,]) {
				break;
			} else if lookahead.peek(Token![?]) {
				input.parse::<Token![?]>()?;
				suffixes.push(ast::Suffix::Unwrap);
			} else if lookahead.peek(Token![.]) {
				input.parse::<Token![.]>()?;
				let lookahead = input.lookahead1();
				if lookahead.peek(Ident) {
					let id = input.parse::<Ident>()?;

					let lookahead = input.lookahead1();
					if lookahead.peek(syn::token::Paren) {
						let arguments;
						parenthesized!(arguments in input);
						let arguments = parse_arguments(&arguments)?;
						suffixes.push(ast::Suffix::FunctionCall(id.to_string(), arguments));
					} else {
						suffixes.push(ast::Suffix::Field(id.to_string()));
					}
				} else if lookahead.peek(LitInt) {
					let tuple_index = input.parse::<LitInt>()?;
					suffixes.push(ast::Suffix::TupleIndex(tuple_index.base10_parse()?));
				} else {
					return Err(lookahead.error());
				}
			} else if lookahead.peek(syn::token::Bracket) {
				let expr;
				bracketed!(expr in input);
				let expr = expr.parse::<ast::Expression>()?;
				suffixes.push(ast::Suffix::ArrayIndex(Box::new(expr)));
			} else {
				return Err(lookahead.error());
			}
		}

		Ok(Self { atom, suffixes })
	}
}

#[cfg(test)]
mod test {
	use super::*;

	use pretty_assertions::assert_eq;
	use quote::quote;

	#[test]
	fn string_lit() {
		let format = "This is a $test".to_string();
		let ast = quote! {#format};
		let result = syn::parse2::<ast::Expression>(ast).unwrap();
		assert_eq!(result.suffixes.len(), 0);
		match result.atom {
			ast::AtomicExpression::LitStr(string, _span) => assert_eq!(string, format),
			_ => panic!(
				"Atom was expected to be a string literal, but is {:#?} instead.",
				result.atom
			),
		}
	}
}
