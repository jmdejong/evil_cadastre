
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::{
	Pos,
	Size,
	locations::Direction,
	entity::Entity,
	resources::{Resource, ResourceCount},
	UserId,
	utils::{partition, partition_by},
	errors::ParseError,
	parse_err
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
	plot_size: Size,
	size: Size,
	tiles: HashMap<Pos, Entity>
}

impl Field {

	pub fn new(plot_size: Size, size: Size) -> Field {
		Self {plot_size, size, tiles: HashMap::new()}
	}
	
	fn keep_location(&self, pos: Pos) -> Pos {
		let plot = pos / self.plot_size;
		let mut base = plot * self.plot_size + self.plot_size / 2;
		if self.plot_size.0 % 2 == 0 {
			base.0 -= plot.1%2;
		}
		if self.plot_size.1 % 2 == 0 {
			base.1 -= plot.0%2;
		}
		base
	}
	
	pub fn claim_first_keep(&mut self, source_pos: Pos, userid: UserId) -> Option<Pos> {
		let pos = self.keep_location(source_pos);
		match self.get(pos) {
			Some(Entity::Keep(_)) => None,
			Some(_) => {panic!("plot without keep: {:?}", pos)}
			None => {
				self.tiles.insert(pos, Entity::Keep(userid));
				Some(pos)
			}
		}
	}
	
	pub fn get(&self, pos: Pos) -> Option<Entity> {
		self.tiles.get(&pos).cloned()
	}
	
	pub fn plot_owner(&self, pos: Pos) -> Option<UserId> {
		match self.get(self.keep_location(pos)){
			Some(Entity::Keep(owner)) => Some(owner.clone()),
			Some(_) => {panic!("plot without keep: {:?}", pos)},
			None => None
		}
	}
	
	pub fn clear_tile(&mut self, pos: Pos) {
		if pos == self.keep_location(pos) {
			return;
		}
		self.tiles.remove(&pos);
	}
	
	pub fn set_tile(&mut self, pos: Pos, ent: Entity) {
		if pos == self.keep_location(pos) {
			return;
		}
		self.tiles.insert(pos, ent);
	}
	
	pub fn set(&mut self, pos: Pos, val: Option<Entity>) {
		match val {
			Some(ent) => self.set_tile(pos, ent),
			None => self.clear_tile(pos)
		}
	}
	
	pub fn tiles_in_plot(&self, pos: Pos) -> Vec<Pos>{
		let mut positions = Vec::new();
		let plot = pos / self.plot_size;
		let keep = self.keep_location(pos);
		for x in plot.0*self.plot_size.0 .. (plot.0+1)*self.plot_size.0 {
			for y in plot.1*self.plot_size.1 .. (plot.1+1)*self.plot_size.1 {
				let tile = Pos(x, y);
				if tile != keep {
					positions.push(tile);
				}
			}
		}
		positions.sort_by_key(|p| p.distance_to(pos));
		positions
	}
	
	pub fn available_resources(&self, source_pos: Pos) -> ResourceCount {
		let mut resources = ResourceCount::default();
		for pos in self.tiles_in_plot(source_pos){
			if let Some(Entity::Stockpile(Some(res))) = self.get(pos) {
				 resources.add_resource(res);
			}
		}
		resources
	}
	
	pub fn change_tile(&mut self, source_pos: Pos, from: Option<Entity>, to: Option<Entity>) -> bool {
		for pos in self.tiles_in_plot(source_pos) {
			if self.get(pos) == from {
				self.set(pos, to);
				return true;
			}
		}
		false
	}
	
	pub fn add_resource(&mut self, pos: Pos, res: Resource) -> bool {
		self.change_tile(pos, Some(Entity::Stockpile(None)), Some(Entity::Stockpile(Some(res))))
	}
	
	
	pub fn neighbour_lane(&mut self, mut pos: Pos, dir: Direction) -> Vec<Pos> {
		let mut lane = Vec::new();
		let dt = dir.to_pos();
		let neighbour = pos / self.plot_size + dt;
		while pos / self.plot_size != neighbour {
			pos = pos + dt;
		}
		while pos / self.plot_size == neighbour {
			lane.push(pos);
			pos = pos + dt;
		}
		lane
	}
	
	pub fn pay(&mut self, pos: Pos, cost: &ResourceCount) -> bool {
		if self.available_resources(pos).can_afford(cost) {
			for res in cost.to_vec() {
				self.change_tile(pos, Some(Entity::Stockpile(Some(res))), Some(Entity::Stockpile(None)));
			}
			return true;
		}
		false
	}
	
	pub fn list_keeps(&self) -> Vec<Pos> {
		let mut keeps = Vec::new();
		for plot_x in 0..self.size.0 {
			for plot_y in 0..self.size.1 {
				keeps.push(self.keep_location(Pos(plot_x, plot_y) * self.plot_size));
			}
		}
		keeps
	}
}



impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "size:{} plot_size:{}\n", self.size, self.plot_size)?;
		for (pos, ent) in self.tiles.iter() {
			write!(f, "{} {}; ", pos, ent)?;
		}
		Ok(())
	}
}

impl FromStr for Field {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (meta, tiles) = partition_by(s, "\n");
		let meta_items = meta.split(' ');
		let mut size = None;
		let mut plot_size = None;
		for meta_item in meta_items {
			let (name, arg) = partition_by(meta_item, ":");
			match name.trim() {
				"size" => {size = Some(Pos::from_str(&arg)?)}
				"plot_size" => {plot_size = Some(Pos::from_str(&arg)?)}
				_ => {}
			}
		}
		Ok(Self{
			tiles: tiles
				.split(';')
				.filter_map(|item| {
					let t = item.trim();
					if t == "" {
						return None;
					}
					Some(t)
				})
				.map(|item| {
					let (pos_s, ent_s) = partition(item);
					Ok((Pos::from_str(&pos_s)?, Entity::from_str(&ent_s)?))
				})
				.collect::<Result<HashMap<Pos, Entity>, Self::Err>>()?,
			size: size.ok_or(parse_err!("No size found for field"))?,
			plot_size: plot_size.ok_or(parse_err!("No plot size found for field"))?,
		})
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	
	#[test]
	fn test_tile_ordering(){
		let field = Field::new(Pos(10,10), Pos(10,10));
		let pos = Pos(6,7);
		let tiles = field.tiles_in_plot(pos);
		for i in 1..tiles.len() {
			assert!(tiles[i].distance_to(pos) >= tiles[i-1].distance_to(pos));
		}
	}
}
