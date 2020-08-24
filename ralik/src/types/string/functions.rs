use anyhow::anyhow;
use num::ToPrimitive;

use crate::error::{Overflow, RuntimeError};
use crate::{Context, TypeHandle, Value};

use super::super::arguments::Arguments;

/**
Get the string as an array of UTF-8 encoded bytes.

```rust
# use ralik::{eval_str, Value};
# let context = ralik::Context::new();
let result = eval_str("\"abc\".as_bytes()", &context).unwrap();
assert_eq!(result, eval_str("[0x61, 0x62, 0x63]", &context).unwrap());
```
*/
pub(crate) fn as_bytes(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	let array = this
		.as_bytes()
		.iter()
		.map(|byte| Value::new_integer(context, *byte))
		.collect::<Result<Vec<_>, _>>()?;

	Ok(Value::new_array(
		context,
		&context.get_integer_type().map_err(|err| anyhow!(err))?,
		array,
	)?)
}

/**
Cloning a string is effectively a no-op in ralik, as all operations have value-semantics.
*/
pub(crate) fn clone(_context: &Context, this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	arguments.check_type(0, this_type).map(|value| value.clone())
}

/**
Case-insensitive comparison for ASCII strings.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"abc\".eq_ignore_ascii_case(\"ABc\")", &context).unwrap();
assert_eq!(result, eval_str("true", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"abc\".eq_ignore_ascii_case(\"abcd\")", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```
*/
pub(crate) fn eq_ignore_ascii_case(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_string(0, context)?;
	let arg = arguments.as_string(1, context)?;
	Ok(Value::new_bool(context, this.eq_ignore_ascii_case(&arg))?)
}

/**
Checks if a string is ASCII-only.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"abc\".is_ascii()", &context).unwrap();
assert_eq!(result, eval_str("true", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"äꞵç\".is_ascii()", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```
*/
pub(crate) fn is_ascii(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_bool(context, this.is_ascii())?)
}

pub(crate) fn is_char_boundary(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_string(0, context)?;
	let arg = arguments
		.as_integer(1, context)?
		.to_usize()
		.ok_or_else(|| Overflow::USize)?;
	Ok(Value::new_bool(context, this.is_char_boundary(arg))?)
}

/**
Checks if a string is empty.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"\".is_empty()", &context).unwrap();
assert_eq!(result, eval_str("true", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"a\".is_empty()", &context).unwrap();
assert_eq!(result, eval_str("false", &context).unwrap());
```
*/
pub(crate) fn is_empty(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_bool(context, this.is_empty())?)
}

/**
Computes the length of a string *in bytes*.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"\".len()", &context).unwrap();
assert_eq!(result, eval_str("0", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"a\".len()", &context).unwrap();
assert_eq!(result, eval_str("1", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"ä\".len()", &context).unwrap();
assert_eq!(result, eval_str("2", &context).unwrap());
```
*/
pub(crate) fn len(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_string(0, context)?;
	Ok(Value::new_integer(context, value.len())?)
}

/**
Repeat the given string `n` times.

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"abc\".repeat(2)", &context).unwrap();
assert_eq!(result, eval_str("\"abcabc\"", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"\".repeat(1)", &context).unwrap();
assert_eq!(result, eval_str("\"\"", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
let result = eval_str("\"abc\".repeat(0)", &context).unwrap();
assert_eq!(result, eval_str("\"\"", &context).unwrap());
```

```rust
# use ralik::eval_str;
# let context = ralik::Context::new();
// the argument must be a non-negative number:
let result = eval_str("\"abc\".repeat(-1)", &context).unwrap_err();
```
*/
pub(crate) fn repeat(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(2)?;
	let this = arguments.as_string(0, context)?;
	let arg = arguments
		.as_integer(1, context)?
		.to_usize()
		.ok_or_else(|| Overflow::USize)?;
	Ok(Value::new_string(context, this.repeat(arg))?)
}

pub(crate) fn to_ascii_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_ascii_lowercase())?)
}

pub(crate) fn to_ascii_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_ascii_uppercase())?)
}

pub(crate) fn to_lowercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_lowercase())?)
}

/**
Converting a string to a string is effectively a no-op in ralik, as all operations have value-semantics.
*/
pub(crate) fn to_string(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let value = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, value.to_string())?)
}

pub(crate) fn to_uppercase(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.to_uppercase())?)
}

pub(crate) fn trim(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.trim())?)
}

pub(crate) fn trim_end(context: &Context, _this_type: &TypeHandle, arguments: &[Value]) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.trim_end())?)
}

pub(crate) fn trim_start(
	context: &Context,
	_this_type: &TypeHandle,
	arguments: &[Value],
) -> Result<Value, RuntimeError> {
	arguments.check_len(1)?;
	let this = arguments.as_string(0, context)?;
	Ok(Value::new_string(context, this.trim_start())?)
}
