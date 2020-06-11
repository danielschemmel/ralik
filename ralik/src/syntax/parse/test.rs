use super::*;

use pretty_assertions::assert_eq;
use quote::quote;

#[test]
fn string_lit() {
	let format = "This is a $test".to_string();
	let ast = quote! {#format};
	let result = syn::parse2::<ast::Expression>(ast).unwrap();

	match result {
		ast::Expression::Atomic(ast::AtomicExpression::LitStr(string, _span)) => assert_eq!(string, format),
		_ => panic!(
			"Expression was expected to be a string literal, but is {:#?} instead.",
			result
		),
	}
}
