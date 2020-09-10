use std::str::FromStr;
use std::fmt;
use std::ops::{Add, Sub, Div, Mul, Rem};

use crate::{
	errors::ParseError,
	parse_err
};


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pos {
	pub x: i32,
	pub y: i32
}

pub type Size = Pos;


impl Pos {

	pub fn new(x: i32, y: i32) -> Self {
		Self {x, y}
	}
	
	pub fn distance_to(self, other: Pos) -> i32 {
		let d = other - self;
		d.x.abs() + d.y.abs()
	}
}

impl Add for Pos {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self::new(self.x + other.x, self.y + other.y)
	}
}
impl Sub for Pos {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self::new(self.x - other.x, self.y - other.y)
	}
}
impl Div for Pos {
	type Output = Self;
	fn div(self, other: Self) -> Self {
		Self::new(self.x / other.x, self.y / other.y)
	}
}
impl Div<i32> for Pos {
	type Output = Self;
	fn div(self, rhs: i32) -> Self::Output {
		Self::new(self.x / rhs, self.y / rhs)
	}
}
impl Rem for Pos {
	type Output = Self;
	fn rem(self, other: Self) -> Self {
		Self::new(self.x % other.x, self.y % other.y)
	}
}
impl Rem<i32> for Pos {
	type Output = Self;
	fn rem(self, rhs: i32) -> Self::Output {
		Self::new(self.x % rhs, self.y % rhs)
	}
}
impl Mul for Pos {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		Self::new(self.x * other.x, self.y * other.y)
	}
}
impl Mul<i32> for Pos {
	type Output = Self;
	fn mul(self, rhs: i32) -> Self::Output {
		Self::new(self.x * rhs, self.y * rhs)
	}
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

		Ok(Self::new(x_fromstr, y_fromstr))
	}
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}



#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Direction {
	North,
	South,
	East,
	West
}

impl Direction {
	pub fn to_pos(&self) -> Pos {
		match self {
			Self::North => Pos::new(0, -1),
			Self::South => Pos::new(0, 1),
			Self::East => Pos::new(1, 0),
			Self::West => Pos::new(-1, 0)
		}
	}
	
	pub fn directions() -> Vec<Self> {
		vec![Self::North, Self::South, Self::East, Self::West]
	}
}


impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::North => "north",
			Self::South => "south",
			Self::East => "east",
			Self::West => "west",
		})
	}
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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_distance() {
		assert_eq!(Pos::new(2, 3).distance_to(Pos::new(1, 5)), 3);
	}
	
	#[test]
	fn test_distance_symetry() {
		for x in 0..22 {
			for y in 0..22 {
				assert_eq!(Pos::new(x, y).distance_to(Pos::new(1,1)), Pos::new(1,1).distance_to(Pos::new(x,y)));
			}
		}
	}
}

