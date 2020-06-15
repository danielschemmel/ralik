use super::{BasicType, BasicTypeBase, TypeKind};

mod functions;
mod ops;

pub type IntegerType = BasicType<IntegerImpl>;
pub const fn name() -> &'static str {
	"Integer"
}

pub struct IntegerImpl;

impl IntegerType {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for IntegerType {
	fn default() -> Self {
		BasicType::from_base_with_functions(
			IntegerImpl,
			vec![
				(crate::ops::NOT, ops::not),
				(crate::ops::NEGATE, ops::negate),
				(crate::ops::NOT, ops::not),
				(crate::ops::MUL, ops::multiply),
				(crate::ops::DIV, ops::divide),
				(crate::ops::REM, ops::remainder),
				(crate::ops::ADD, ops::add),
				(crate::ops::SUB, ops::subtract),
				(crate::ops::SHL, ops::shift_left),
				(crate::ops::SHR, ops::shift_right),
				(crate::ops::BIT_AND, ops::bit_and),
				(crate::ops::BIT_OR, ops::bit_or),
				(crate::ops::BIT_XOR, ops::bit_xor),
				(crate::ops::EQUAL, ops::equal),
				(crate::ops::NOT_EQUAL, ops::not_equal),
				(crate::ops::LESS, ops::less),
				(crate::ops::LESS_OR_EQUAL, ops::less_or_equal),
				(crate::ops::GREATER, ops::greater),
				(crate::ops::GREATER_OR_EQUAL, ops::greater_or_equal),
				("abs", functions::abs),
				//("checked_div", functions::checked_div),
				("clone", functions::clone),
				("is_negative", functions::is_negative),
				("is_positive", functions::is_positive),
				("pow", functions::pow),
				("signum", functions::signum),
				("to_string", functions::to_string),
			],
		)
	}
}

impl BasicTypeBase for IntegerImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Integer
	}
}
