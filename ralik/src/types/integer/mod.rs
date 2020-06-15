use super::{BasicFunctionStore, BasicType, BasicTypeBase, TypeKind};

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
		BasicType::from_base(IntegerImpl)
	}
}

impl BasicTypeBase for IntegerImpl {
	fn name(&self) -> &str {
		self::name()
	}

	fn kind(&self) -> TypeKind {
		TypeKind::Integer
	}

	fn register_functions(&self, functions: &mut BasicFunctionStore) {
		functions.insert(crate::ops::NOT.into(), ops::not);
		functions.insert(crate::ops::NEGATE.into(), ops::negate);
		functions.insert(crate::ops::NOT.into(), ops::not);
		functions.insert(crate::ops::MUL.into(), ops::multiply);
		functions.insert(crate::ops::DIV.into(), ops::divide);
		functions.insert(crate::ops::REM.into(), ops::remainder);
		functions.insert(crate::ops::ADD.into(), ops::add);
		functions.insert(crate::ops::SUB.into(), ops::subtract);
		functions.insert(crate::ops::SHL.into(), ops::shift_left);
		functions.insert(crate::ops::SHR.into(), ops::shift_right);
		functions.insert(crate::ops::BIT_AND.into(), ops::bit_and);
		functions.insert(crate::ops::BIT_OR.into(), ops::bit_or);
		functions.insert(crate::ops::BIT_XOR.into(), ops::bit_xor);
		functions.insert(crate::ops::EQUAL.into(), ops::equal);
		functions.insert(crate::ops::NOT_EQUAL.into(), ops::not_equal);
		functions.insert(crate::ops::LESS.into(), ops::less);
		functions.insert(crate::ops::LESS_OR_EQUAL.into(), ops::less_or_equal);
		functions.insert(crate::ops::GREATER.into(), ops::greater);
		functions.insert(crate::ops::GREATER_OR_EQUAL.into(), ops::greater_or_equal);

		functions.insert("abs".into(), functions::abs);
		//functions.insert("checked_div".into(), functions::checked_div);
		functions.insert("clone".into(), functions::clone);
		functions.insert("is_negative".into(), functions::is_negative);
		functions.insert("is_positive".into(), functions::is_positive);
		functions.insert("pow".into(), functions::pow);
		functions.insert("signum".into(), functions::signum);
		functions.insert("to_string".into(), functions::to_string);
	}
}
