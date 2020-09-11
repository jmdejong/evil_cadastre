
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ParseError {
	pub msg: String
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ParseError({})", &self.msg)
	}
}

#[macro_export]
macro_rules! parse_err {
	($($description:tt)*) => {crate::errors::ParseError{msg: format!($($description)*)}}
}
