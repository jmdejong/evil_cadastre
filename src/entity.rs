
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
	
	Keep(UserId),
	
	Construction(BuildingType),
	
	// Units
	Raider,
	Warrior,
	
	// Buildings
	Farm,
	Woodcutter,
// 	GuardTower,
	Lair,
	Stockpile(Option<Resource>),
	Road,
	Quarry,
	
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
	pub mortal: bool
}



impl Entity {
	
	pub fn properties(&self) -> EntityProperties {
	
		let unit = EntityProperties{removable: false, destructible: false, mortal: true, stopping: true};
		let building = EntityProperties{removable: true, destructible: true, mortal: false, stopping: true};
		let ambient = EntityProperties{removable: false, destructible: false, mortal: false, stopping: false};
		match self {
			Self::Keep(_) => EntityProperties{removable: false, destructible: false, mortal: false, stopping: true},
			Self::Raider => unit,
			Self::Warrior => unit,
			Self::Farm => building,
			Self::Woodcutter => building,
// 			Self::Guardpost => EntityProperties{removable: true},
			Self::Quarry => building,
			Self::Lair => building,
			Self::Stockpile(_) => building,
			Self::Construction(_) => EntityProperties{removable: true, destructible: true, mortal: false, stopping: false},
			Self::Road => EntityProperties{removable: true, destructible: true, mortal: false, stopping: false},
			Self::Forest => ambient,
			Self::Swamp => ambient,
			Self::Rock => ambient
		}
	}
}


impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Keep(user) => format!("keep:{}", user.0),
			Self::Raider => "raider".to_string(),
			Self::Warrior => "warrior".to_string(),
			Self::Farm => "farm".to_string(),
			Self::Woodcutter => "woodcutter".to_string(),
			Self::Quarry => "quarry".to_string(),
			Self::Lair => "lair".to_string(),
			Self::Stockpile(Some(res)) => format!("stockpile:{}", res),
			Self::Stockpile(None) => "stockpile".to_string(),
			Self::Construction(building) => format!("construction:{}", building),
			Self::Road => "road".to_string(),
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
			("keep", Some(user)) => Self::Keep(UserId(user.to_string())),
			("raider", None) => Self::Raider,
			("warrior", None) => Self::Warrior,
			("farm", None) => Self::Farm,
			("woodcutter", None) => Self::Woodcutter,
			("quarry", None) => Self::Quarry,
			("lair", None) => Self::Lair,
			("stockpile", None) => Self::Stockpile(None),
			("stockpile", Some(res)) => Self::Stockpile(Some(Resource::from_str(res)?)),
			("construction", Some(building)) => Self::Construction(BuildingType::from_str(building)?),
			("road", None) => Self::Road,
			("forest", None) => Self::Forest,
			("swamp", None) => Self::Swamp,
			("rock", None) => Self::Rock,
			_ => {return Err(parse_err!("Invalid entity '{}'", s))}
		})
	}
}
