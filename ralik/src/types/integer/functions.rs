use num::{Signed, ToPrimitive};

use crate::error::{Overflow, RuntimeError};
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

/**
Compute the absolute value of a number.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("(-1).abs()", &context).unwrap();
assert_eq!(result, eval_str("1", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("123456789012345678901234567890.abs()", &context).unwrap();
assert_eq!(result, eval_str("123456789012345678901234567890", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("(-123456789012345678901234567890).abs()", &context).unwrap();
assert_eq!(result, eval_str("123456789012345678901234567890", &context).unwrap());
```
*/
pub(crate) fn abs(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0, context)?;
	Ok(Value::new_integer(context, this.abs())?)
}

/*
pub(crate) fn checked_div(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let lhs = arguments.as_integer(0, context)?;
	let rhs = arguments.as_integer(1, context)?;
	let result = lhs.checked_div(rhs);
	Ok(Value::Option(result.map(|value| Box::new(Value::Integer(value)))))
}
*/

/**
Cloning an int is effectively a no-op in ralik, as all operations have value-semantics.
*/
pub(crate) fn clone(_context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	arguments.check_type(0, this_type).map(|value| value.clone())
}

/**
Checks if a number is negative, i.e., less than 0.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("(-1).is_negative()", &context).unwrap();
assert_eq!(result, eval_str("true", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("1.is_negative()", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("0.is_negative()", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```
*/
pub(crate) fn is_negative(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0, context)?;
	Ok(Value::new_bool(context, this.is_negative())?)
}

/**
Checks if a number is positive, i.e., greater than 0.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("1.is_positive()", &context).unwrap();
assert_eq!(result, eval_str("true", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("(-1).is_positive()", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("0.is_positive()", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```
*/
pub(crate) fn is_positive(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0, context)?;
	Ok(Value::new_bool(context, this.is_positive())?)
}

/**
Raises a number to a given power. Exponents are limited to unsigned 32 bit integers, even for the trivial bases `0` and
`1`. Note that `0.pow(0)

Integers are arbitrary-precision in ralik, but it is fairly easy to exhaust the available main memory with this
function.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("2.pow(3)", &context).unwrap();
assert_eq!(result, eval_str("8", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("2.pow(0)", &context).unwrap();
assert_eq!(result, eval_str("1", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("0.pow(0)", &context).unwrap();
assert_eq!(result, eval_str("1", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("0.pow(1)", &context).unwrap();
assert_eq!(result, eval_str("0", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("0.pow(42)", &context).unwrap();
assert_eq!(result, eval_str("0", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("1.pow(42)", &context).unwrap();
assert_eq!(result, eval_str("1", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("1.pow(-2)", &context).unwrap_err();
```
*/
pub(crate) fn pow(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_integer(0, context)?;
	let arg = arguments
		.as_integer(1, context)?
		.to_u32()
		.ok_or_else(|| Overflow::U32)?;
	Ok(Value::new_integer(context, this.pow(arg))?)
}

/**
Returns `1` if the value is greater than `0`, `0` if the value is `0` and `-1` if the value is less than `0`.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("42.signum()", &context).unwrap();
assert_eq!(result, eval_str("1", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("0.signum()", &context).unwrap();
assert_eq!(result, eval_str("0", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("(-42).signum()", &context).unwrap();
assert_eq!(result, eval_str("-1", &context).unwrap());
```
*/
pub(crate) fn signum(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0, context)?;
	Ok(Value::new_integer(context, this.signum())?)
}

/**
Converts a number to a string.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("1_000.to_string()", &context).unwrap();
assert_eq!(result, eval_str("\"1000\"", &context).unwrap());
```
*/
pub(crate) fn to_string(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_integer(0, context)?;
	Ok(Value::new_string(context, this.to_string())?)
}
