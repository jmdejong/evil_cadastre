use std::str::FromStr;

use crate::{
	errors::ParseError,
	parse_err
};


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Pos {
	pub x: i32,
	pub y: i32
}

impl FromStr for Pos {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let coords: Vec<&str> = s.split(',').collect();
		if coords.len() != 2 {
			return Err(parse_err!("Position must be 2 integers separated by a comma. Found '{}'", s));
		}
		let x_fromstr = coords[0].parse::<i32>().map_err(|e|parse_err!("Invalid Position '{}': {}", s, e.to_string()))?;
		let y_fromstr = coords[1].parse::<i32>().map_err(|e|parse_err!("Invalid Position '{}': {}", s, e.to_string()))?;

		Ok(Pos { x: x_fromstr, y: y_fromstr })
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Direction {
	North,
	South,
	East,
	West
}


impl FromStr for Direction {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"north" => Ok(Self::North),
			"south" => Ok(Self::South),
			"east" => Ok(Self::East),
			"west" => Ok(Self::West),
			_ => Err(parse_err!("Invalid direction '{}'", s))
		}
	}
}
