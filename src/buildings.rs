
use std::str::FromStr;

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
// 	Guardpost,
	Stockpile,
	Lair
}

use Resource::*;

impl BuildingType {
	
	pub fn cost_result(&self) -> (ResourceCount, Entity) {
		let (cost, result) = match self {
			Self::Woodcutter => (vec![], Entity::Woodcutter),
			Self::Farm => (vec![Wood], Entity::Farm),
			Self::Stockpile => (vec![], Entity::Stockpile(None)),
			Self::Lair => (vec![Wood, Wood, Wood], Entity::Lair),
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
// 			"guardpost" => Self::guardpost,
			"lair" => Self::Lair,
			"stockpile" => Self::Stockpile,
			_ => {return Err(parse_err!("Invalid building '{}'", s))}
		})
	}
}

