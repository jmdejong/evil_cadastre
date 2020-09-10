
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::{
	errors::ParseError,
	parse_err,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
	Food,
	Wood,
	Stone,
	Iron
}


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ResourceCount {
	pub resources: HashMap<Resource, usize>
}

impl ResourceCount {

	pub fn from_vec(r: &[Resource]) -> ResourceCount {
		let mut count = ResourceCount::default();
		for res in r {
			count.add_resource(*res);
		}
		count
	}

	pub fn add_resource(&mut self, res: Resource) {
		let amount: &mut usize = self.resources.entry(res).or_insert(0);
		*amount += 1;
	}
	
	pub fn can_afford(&self, cost: &ResourceCount) -> bool {
		for (res, amount) in cost.resources.iter(){
			if amount > self.resources.get(res).unwrap_or(&0) {
				return false;
			}
		}
		return true;
	}
	
	pub fn to_vec(&self) -> Vec<Resource> {
		let mut resvec = Vec::new();
		for (res, amount) in self.resources.iter(){
			for _ in 0..*amount {
				resvec.push(*res);
			}
		}
		resvec
	}
}




impl fmt::Display for Resource {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Food => "food",
			Self::Wood => "wood",
			Self::Stone => "stone",
			Self::Iron => "iron",
		})
	}
}

impl FromStr for Resource {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"food" => Ok(Self::Food),
			"wood" => Ok(Self::Wood),
			"stone" => Ok(Self::Stone),
			"iron" => Ok(Self::Iron),
			_ => Err(parse_err!("Invalid resource '{}'", s))
		}
	}
}
