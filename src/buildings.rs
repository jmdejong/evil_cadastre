
use std::str::FromStr;

use crate::{
	errors::ParseError,
	parse_err
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Building {
	WoodCutter,
	Farm,
	GuardTower
}



impl FromStr for Building {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s.to_lowercase().as_str() {
// 			"build" => Self::Build(
			"woodcutter" => Self::WoodCutter,
			"farm" => Self::Farm,
			"guardtower" => Self::GuardTower,
			_ => {return Err(parse_err!("Invalid building: '{}'", s))}
		})
	}
}
