
use std::fmt;
use std::str::FromStr;

use crate::{
	UserId,
	buildings::BuildingType,
	resources::Resource,
	errors::ParseError,
	parse_err,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entity {
	
	Capital(UserId),
	Keep(UserId),
	
	Construction(BuildingType),
	
	// Units
	Raider,
	Warrior,
	
	// Production buildings
	Farm,
	Woodcutter,
	Quarry,
	// Unit training buildings
	Lair,
	Barracks,
	// Special buildings
	Stockpile(Option<Resource>),
	Road,
	Tradepost,
	Scoutpost,
	
	// Ambient
	Forest,
	Swamp,
	Rock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityProperties {
	pub removable: bool,
	pub destructible: bool,
	pub stopping: bool,
	pub mortal: bool,
	pub unit: bool
}



impl Entity {
	
	pub fn properties(&self) -> EntityProperties {
	
		let unit = EntityProperties{removable: false, destructible: false, mortal: true, stopping: true, unit: true};
		let building = EntityProperties{removable: true, destructible: true, mortal: false, stopping: true, unit: false};
		let ambient = EntityProperties{removable: false, destructible: false, mortal: false, stopping: false, unit: false};
		let small = EntityProperties{removable: true, destructible: true, mortal: false, stopping: false, unit: false};
		match self {
			Self::Capital(_) => EntityProperties{removable: false, destructible: false, mortal: false, stopping: true, unit: false},
			Self::Keep(_) => EntityProperties{removable: false, destructible: false, mortal: false, stopping: true, unit: false},
			Self::Raider => unit,
			Self::Warrior => unit,
			Self::Farm => building,
			Self::Woodcutter => building,
			Self::Quarry => building,
			Self::Lair => building,
			Self::Barracks => building,
			Self::Stockpile(_) => building,
			Self::Construction(_) => small,
			Self::Road => small,
			Self::Tradepost => small,
			Self::Scoutpost => building,
			Self::Forest => ambient,
			Self::Swamp => ambient,
			Self::Rock => ambient
		}
	}
}


impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Capital(user) => format!("capital:{}", user.0),
			Self::Keep(user) => format!("keep:{}", user.0),
			Self::Raider => "raider".to_string(),
			Self::Warrior => "warrior".to_string(),
			Self::Farm => "farm".to_string(),
			Self::Woodcutter => "woodcutter".to_string(),
			Self::Quarry => "quarry".to_string(),
			Self::Lair => "lair".to_string(),
			Self::Barracks => "barracks".to_string(),
			Self::Stockpile(Some(res)) => format!("stockpile:{}", res),
			Self::Stockpile(None) => "stockpile".to_string(),
			Self::Construction(building) => format!("construction:{}", building),
			Self::Road => "road".to_string(),
			Self::Tradepost => "tradepost".to_string(),
			Self::Scoutpost => "scoutpost".to_string(),
			Self::Forest => "forest".to_string(),
			Self::Swamp => "swamp".to_string(),
			Self::Rock => "rock".to_string(),
		})
	}
}

impl FromStr for Entity {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = s.to_lowercase();
		let mut c = l.splitn(2, ':');
		let typ = c.next().unwrap();
		let arg = c.next();
		Ok(match (typ, arg) {
			("capital", Some(user)) => Self::Capital(UserId(user.to_string())),
			("keep", Some(user)) => Self::Keep(UserId(user.to_string())),
			("raider", None) => Self::Raider,
			("warrior", None) => Self::Warrior,
			("farm", None) => Self::Farm,
			("woodcutter", None) => Self::Woodcutter,
			("quarry", None) => Self::Quarry,
			("lair", None) => Self::Lair,
			("Barracks", None) => Self::Barracks,
			("stockpile", None) => Self::Stockpile(None),
			("stockpile", Some(res)) => Self::Stockpile(Some(Resource::from_str(res)?)),
			("construction", Some(building)) => Self::Construction(BuildingType::from_str(building)?),
			("road", None) => Self::Road,
			("tradepost", None) => Self::Tradepost,
			("scoutpost", None) => Self::Scoutpost,
			("forest", None) => Self::Forest,
			("swamp", None) => Self::Swamp,
			("rock", None) => Self::Rock,
			_ => {return Err(parse_err!("Invalid entity '{}'", s))}
		})
	}
}
