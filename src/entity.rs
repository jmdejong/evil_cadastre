
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
	
	// Buildings
	Farm,
	Woodcutter,
// 	GuardTower,
	Lair,
	Stockpile(Option<Resource>)
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
		match self {
			Entity::Keep(_) => EntityProperties{removable: false, destructible: false, mortal: false, stopping: true},
			Entity::Raider => unit,
			Entity::Farm => building,
			Entity::Woodcutter => building,
// 			Entity::Guardpost => EntityProperties{removable: true},
			Entity::Lair => building,
			Entity::Stockpile(_) => building,
			Entity::Construction(_) => EntityProperties{removable: true, destructible: true, mortal: false, stopping: false}
		}
	}
}


impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Keep(user) => format!("keep:{}", user.0),
			Self::Raider => "raider".to_string(),
			Self::Farm => "farm".to_string(),
			Self::Woodcutter => "woodcutter".to_string(),
			Self::Lair => "lair".to_string(),
			Self::Stockpile(Some(res)) => format!("stockpile:{}", res),
			Self::Stockpile(None) => format!("stockpile"),
			Self::Construction(building) => format!("construction:{}", building)
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
			("farm", None) => Self::Farm,
			("woodcutter", None) => Self::Woodcutter,
			("lair", None) => Self::Lair,
			("stockpile", None) => Self::Stockpile(None),
			("stockpile", Some(res)) => Self::Stockpile(Some(Resource::from_str(res)?)),
			("construction", Some(building)) => Self::Construction(BuildingType::from_str(building)?),
			_ => {return Err(parse_err!("Invalid entity '{}'", s))}
		})
	}
}
