

use crate::{
	UserId,
	buildings::BuildingType,
	resources::Resource
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
