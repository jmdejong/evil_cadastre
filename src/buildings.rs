

use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;
use std::str::FromStr;

use crate::{
	entity::Entity,
	resources::{Resource, ResourceCount},
	errors::ParseError,
	parse_err
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum BuildingType {
	Woodcutter,
	Farm,
	Quarry,
// 	Guardpost,
	Stockpile,
	Lair,
	Barracks,
	Road,
	Tradepost,
	Scoutpost,
	Ram,
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
			Self::Tradepost => (vec![Wood, Wood, Stone], Entity::Tradepost),
			Self::Scoutpost => (vec![Wood, Wood, Wood, Wood, Wood, Stone], Entity::Scoutpost),
			Self::Ram => (vec![Wood, Wood, Wood, Wood, Wood, Wood, Food], Entity::Ram),
		};
		(ResourceCount::from_vec(&cost), result)
	}
}


impl FromStr for BuildingType {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let name = s.to_lowercase();
		for building in BuildingType::iter(){
			if name == building.to_string(){
				return Ok(building);
			}
		}
		Err(parse_err!("Invalid building '{}'", s))
// 		Ok(match s.to_lowercase().as_str() {
// 			"woodcutter" => Self::Woodcutter,
// 			"farm" => Self::Farm,
// 			"quarry" => Self::Quarry,
// // 			"guardpost" => Self::guardpost,
// 			"lair" => Self::Lair,
// 			"barracks" => Self::Barracks,
// 			"stockpile" => Self::Stockpile,
// 			"road" => Self::Road,
// 			"tradepost" => Self::Tradepost,
// 			"scoutpost" => Self::Scoutpost,
// 			_ => {return Err(parse_err!("Invalid building '{}'", s))}
// 		})
	}
}
// 
// impl fmt::Display for BuildingType {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		write!(f, "{}", match self {
// 			Self::Woodcutter => "woodcutter",
// 			Self::Farm => "farm",
// 			Self::Quarry => "quarry",
// 			Self::Lair => "lair",
// 			Self::Barracks => "barracks",
// 			Self::Stockpile => "stockpile",
// 			Self::Road => "road",
// 			Self::Tradepost => "tradepost",
// 			Self::Scoutpost => "scoutpost",
// 		})
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_serialisation(){
		for building in BuildingType::iter(){
			let a = building.to_string();
			let b = BuildingType::from_str(&a).unwrap();
			let c = b.to_string();
			assert_eq!(building, b);
			assert_eq!(a, c);
		}
	}
}

