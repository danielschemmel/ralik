use num_bigint::Sign;
use num_traits::ToPrimitive;

use crate::{Context, Value, CallError};

use super::super::arguments::Arguments;
use super::super::{CallError, Overflow};

pub(crate) fn unwrap(context: &Context, arguments: &[Value]) -> Result<Value, CallError> {
	arguments.check_len(1)?;
	let value = arguments.as_option(0)?;
	unimplemented!()
}
