use std::str::FromStr;

use crate::{
	Pos,
	locations::Direction,
	partition,
	errors::ParseError,
	parse_err,
	buildings::BuildingType
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Command {
	pub pos: Pos,
	pub action: Action
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Action{
	Build(BuildingType),
	Move(Pos),
	Attack(Direction),
	Remove,
	Use
}


impl FromStr for Action {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (commtype, arg) = partition(s);
		Ok(match commtype.to_lowercase().as_str() {
			"build" => Self::Build(BuildingType::from_str(&arg)?),
			"move" => Self::Move(Pos::from_str(&arg)?),
			"attack" => Self::Attack(Direction::from_str(&arg)?),
			"remove" => Self::Remove,
			"use" => Self::Use,
			_ => {return Err(parse_err!("Invalid action '{}'", commtype))}
		})
	}
}


impl FromStr for Command {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (pos, action) = partition(s);
		Ok(Self{pos: Pos::from_str(&pos)?, action: Action::from_str(&action)?})
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	macro_rules! a {
		($command: expr, $out: expr) => {assert_eq!(Command::from_str($command), $out)}
	}
	macro_rules! e {
		($command: expr, $err: expr) => {a!($command, Err(ParseError{msg: $err.to_string()}))}
	}
	macro_rules! c {
		($command: expr, ($x: expr, $y: expr), $action: expr) => {
			a!($command, Ok(Command{pos: Pos($x, $y), action: $action}));
		}
	}
	
	#[test]
	fn test_command_parsing() {
		c!("0,0 build woodcutter", (0, 0), Action::Build(BuildingType::Woodcutter));
		c!("3,3 build farm", (3, 3), Action::Build(BuildingType::Farm));
		c!("3,5 move 3,0", (3, 5), Action::Move(Pos(3, 0)));
		c!("-1,6 build farm", (-1, 6), Action::Build(BuildingType::Farm));
		e!("1,1,1 build farm", "Position must be 2 integers separated by a comma. Found '1,1,1'");
		e!("invalid build farm", "Position must be 2 integers separated by a comma. Found 'invalid'");
		e!("1,1 build invalid", "Invalid building 'invalid'");
		e!("1,1 build", "Invalid building ''");
		e!("1,1 invalid", "Invalid action 'invalid'");
		e!("1,1 move invalid", "Position must be 2 integers separated by a comma. Found 'invalid'");
		e!("1,1", "Invalid action ''");
  		c!("8,5 build lair", (8, 5), Action::Build(BuildingType::Lair));
		c!("7,3 attack west", (7, 3), Action::Attack(Direction::West));
		e!("1,1 attack invalid", "Invalid direction 'invalid'");
		c!("6,6 remove", (6, 6), Action::Remove);
		c!("7,4 use", (7, 4), Action::Use);
	}
}
