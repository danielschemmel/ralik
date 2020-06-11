use num_bigint::BigInt;

mod debug;

#[derive(Clone)]
pub enum Value {
	Bool(bool),
	Char(char),
	Integer(BigInt),
	String(String),
}
