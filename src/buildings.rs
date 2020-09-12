
use std::str::FromStr;
use std::fmt;

use crate::{
	entity::Entity,
	resources::{Resource, ResourceCount},
	errors::ParseError,
	parse_err
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum BuildingType {
	Woodcutter,
	Farm,
	Quarry,
// 	Guardpost,
	Stockpile,
	Lair,
	Barracks,
	Road
}

use Resource::*;

impl BuildingType {
	
	pub fn cost_result(&self) -> (ResourceCount, Entity) {
		let (cost, result) = match self {
			Self::Woodcutter => (vec![], Entity::Woodcutter),
			Self::Farm => (vec![Wood], Entity::Farm),
			Self::Quarry => (vec![Wood, Wood, Wood, Wood], Entity::Quarry),
			Self::Stockpile => (vec![], Entity::Stockpile(None)),
			Self::Lair => (vec![Wood, Wood, Wood], Entity::Lair),
			Self::Barracks => (vec![Wood, Wood, Wood, Wood, Stone, Stone, Stone], Entity::Barracks),
			Self::Road => (vec![Wood, Stone], Entity::Road),
		};
		(ResourceCount::from_vec(&cost), result)
	}
}


impl FromStr for BuildingType {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s.to_lowercase().as_str() {
			"woodcutter" => Self::Woodcutter,
			"farm" => Self::Farm,
			"quarry" => Self::Quarry,
// 			"guardpost" => Self::guardpost,
			"lair" => Self::Lair,
			"barracks" => Self::Barracks,
			"stockpile" => Self::Stockpile,
			"road" => Self::Road,
			_ => {return Err(parse_err!("Invalid building '{}'", s))}
		})
	}
}

impl fmt::Display for BuildingType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Woodcutter => "woodcutter",
			Self::Farm => "farm",
			Self::Quarry => "quarry",
			Self::Lair => "lair",
			Self::Barracks => "barracks",
			Self::Stockpile => "stockpile",
			Self::Road => "road"
		})
	}
}

