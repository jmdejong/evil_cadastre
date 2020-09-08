use std::str::FromStr;

use crate::{
	Pos,
	Direction,
	buildings::Building,
	partition,
	errors::ParseError,
	parse_err
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PlayerCommand {
	pos: Pos,
	action: Action
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Action{
	Build(Building),
	Move(Pos),
	Attack(Direction),
	Remove,
	Recruit
}

impl FromStr for Action {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (commtype, arg) = partition(s);
		Ok(match commtype.to_lowercase().as_str() {
			"build" => Self::Build(Building::from_str(&arg)?),
			"move" => Self::Move(Pos::from_str(&arg)?),
			"attack" => Self::Attack(Direction::from_str(&arg)?),
			"remove" => Self::Remove,
			"recruit" => Self::Recruit,
			_ => {return Err(parse_err!("Invalid action '{}'", commtype))}
		})
	}
}


impl FromStr for PlayerCommand {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (pos, action) = partition(s);
		Ok(PlayerCommand{pos: Pos::from_str(&pos)?, action: Action::from_str(&action)?})
	}
}
