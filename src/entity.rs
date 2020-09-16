
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
	Ram,
	
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntityProperties {
	pub removable: bool,
	pub destructible: bool,
	pub stopping: bool,
	pub mortal: bool,
	pub movable: bool,
	pub strong: bool,
	pub defender: bool
}

macro_rules! props {
	($($prop: ident),*) => {{
		#[allow(unused_mut)]
		let mut properties = EntityProperties::default();
		$(
			properties.$prop = true;
		)*
		properties
	}}
}

impl Entity {
	
	pub fn properties(&self) -> EntityProperties {
	
		let unit = props!(mortal, stopping, movable, defender);
		let building = props!(removable, destructible, stopping);
		let small = props!(removable, destructible);
		match self {
			Self::Capital(_) => props!(destructible, strong, stopping),
			Self::Keep(_) => props!(destructible, strong, stopping),
			Self::Raider => unit,
			Self::Warrior => unit,
			Self::Ram => props!(removable, destructible, mortal, stopping),
			Self::Farm => building,
			Self::Woodcutter => building,
			Self::Quarry => building,
			Self::Lair => building,
			Self::Barracks => building,
			Self::Stockpile(_) => props!(removable),
			Self::Construction(_) => small,
			Self::Road => small,
			Self::Tradepost => small,
			Self::Scoutpost => building,
			Self::Forest => props!(),
			Self::Swamp => props!(),
			Self::Rock => props!()
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
			Self::Ram => "ram".to_string(),
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
		let mut c = s.splitn(2, ':');
		let typ = c.next().unwrap().to_lowercase();
		let arg = c.next();
		Ok(match (typ.as_str(), arg) {
			("capital", Some(user)) => Self::Capital(UserId(user.to_string())),
			("keep", Some(user)) => Self::Keep(UserId(user.to_string())),
			("raider", None) => Self::Raider,
			("warrior", None) => Self::Warrior,
			("ram", None) => Self::Ram,
			("farm", None) => Self::Farm,
			("woodcutter", None) => Self::Woodcutter,
			("quarry", None) => Self::Quarry,
			("lair", None) => Self::Lair,
			("barracks", None) => Self::Barracks,
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


#[cfg(test)]
mod tests {
	use super::*;
	use super::Entity::*;
	use Resource::*;
	
	fn user(name: &str) -> UserId {
		UserId(name.to_string())
	}
	
	#[test]
	fn test_serialisation(){
		
		let entities = vec![
			Capital(user("abc")),
			Capital(user("")),
			Capital(user("ABC")),
			Capital(user("evil")),
			Keep(user("abc")),
			Keep(user("")),
			Keep(user("ABC")),
			Keep(user("evil")),
			Keep(user(":e:v:i:l")),
			Raider,
			Warrior,
			Ram,
			Farm,
			Woodcutter,
			Quarry,
			Lair,
			Barracks,
			Stockpile(None),
			Stockpile(Some(Wood)),
			Stockpile(Some(Food)),
			Stockpile(Some(Stone)),
			Stockpile(Some(Iron)),
			Construction(BuildingType::Barracks),
			Road,
			Tradepost,
			Scoutpost,
			Forest,
			Swamp,
			Rock,
		];
		
		for ent in entities {
			let a = ent.to_string();
			let b = Entity::from_str(&a).unwrap();
			let c = b.to_string();
			assert_eq!(ent, b);
			assert_eq!(a, c);
		}
	}
}

