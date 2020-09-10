
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
	
	pub fn plant_ambience(&mut self) {
		for keep in self.list_keeps() {
			let tiles: Vec<Pos> = self.tiles_in_plot(keep).into_iter().filter(|tile| self.get(*tile) == None).collect();
			let plot = keep / self.plot_size;
			let id = plot.x * 55217 + plot.y * 82487;
			self.set_tile(tiles[id as usize % tiles.len()], Entity::Forest);
		}
	}
	
	pub fn keep_location(&self, pos: Pos) -> Pos {
		let plot = pos / self.plot_size;
		let mut base = plot * self.plot_size + self.plot_size / 2;
		if self.plot_size.x % 2 == 0 {
			base.x -= plot.y%2;
		}
		if self.plot_size.y % 2 == 0 {
			base.y -= plot.x%2;
		}
		base
	}
	
	pub fn claim_first_keep(&mut self, source_pos: Pos, userid: UserId) -> Option<Pos> {
		if !self.is_valid(source_pos){
			return None;
		}
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
		for x in plot.x*self.plot_size.x .. (plot.x+1)*self.plot_size.x {
			for y in plot.y*self.plot_size.y .. (plot.y+1)*self.plot_size.y {
				let tile = Pos::new(x, y);
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
	
	pub fn find(&self, source_pos: Pos, ent: Option<Entity>) -> Option<Pos> {
		for pos in self.tiles_in_plot(source_pos) {
			if self.get(pos) == ent {
				return Some(pos);
			}
		}
		None
	}
	
	pub fn change_tile(&mut self, source_pos: Pos, from: Option<Entity>, to: Option<Entity>) -> bool {
		if let Some(pos) = self.find(source_pos, from){
			self.set(pos, to);
			true
		} else {
			false
		}
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
		for plot_x in 0..self.size.x {
			for plot_y in 0..self.size.y {
				keeps.push(self.keep_location(Pos::new(plot_x, plot_y) * self.plot_size));
			}
		}
		keeps
	}
	
	pub fn is_valid(&self, pos: Pos) -> bool {
		let size = self.size * self.plot_size;
		pos.x >= 0 && pos.y >=0 && pos.x < size.x && pos.y < size.y
	}
	
	pub fn across_border(&self, pos: Pos) -> Option<Pos> {
		let keep = self.keep_location(pos);
		let mut crossings = vec![];
		for dir in Direction::directions(){
			let p = pos + dir.to_pos();
			if self.keep_location(p) != keep {
				crossings.push(p)
			}
		}
		if crossings.len() == 1 {
			Some(crossings[0])
		} else {
			None
		}
	}
	
	pub fn cross_pos(&self, to: Pos) -> Option<Pos> {
		let pos = self.find(self.across_border(to)?, None)?;
		if self.plot_owner(pos) == self.plot_owner(to) {
			Some(pos)
		} else {
			None
		}
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
		let field = Field::new(Pos::new(10,10), Pos::new(10,10));
		let pos = Pos::new(6,7);
		let tiles = field.tiles_in_plot(pos);
		for i in 1..tiles.len() {
			assert!(tiles[i].distance_to(pos) >= tiles[i-1].distance_to(pos));
		}
	}
}
