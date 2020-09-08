

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ParseError {
	pub msg: String
}

#[macro_export]
macro_rules! parse_err {
	($($description:tt)*) => {crate::errors::ParseError{msg: format!($($description)*)}}
}
