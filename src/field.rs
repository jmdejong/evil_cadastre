
use std::collections::HashMap;

use crate::{
	Pos,
	locations::Direction,
	entity::Entity,
	resources::{Resource, ResourceCount},
	UserId
};

// struct Plot{
// 	pub owner: UserId,

// pub struct Tile {
// 	pub occupant: Option<Box<dyn Entity>>
// }

pub struct Field {
	plot_size: (i32, i32),
	size: (i32, i32),
	tiles: HashMap<Pos, Entity>
}

impl Field {

	pub fn new(plot_size: (i32, i32), size: (i32, i32)) -> Field {
		Self {plot_size, size, tiles: HashMap::new()}
	}
	
	fn keep_location(&self, pos: Pos) -> Pos {
		let plot_idx = pos.0 / self.plot_size.0;
		let plot_idy = pos.1 / self.plot_size.1;
		let base_x = plot_idx * self.plot_size.0 + self.plot_size.0 / 2;
		let base_y = plot_idy * self.plot_size.1 + self.plot_size.1 / 2;
		let offset_x = (1 - self.plot_size.0 % 2) * -plot_idx % 2;
		let offset_y = (1 - self.plot_size.1 % 2) * -plot_idy % 2;
		Pos(base_x + offset_x, base_y + offset_y)
	}
	
	pub fn get(&self, pos: Pos) -> Option<Entity> {
		self.tiles.get(&pos).cloned()
	}
	
	pub fn plot_owner(&self, pos: Pos) -> Option<UserId> {
		match self.get(self.keep_location(pos)){
			Some(Entity::Keep(owner)) => owner.clone(),
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
	
	fn tiles_in_plot(&self, pos: Pos) -> Vec<Pos>{
		let mut positions = Vec::new();
		let plot_idx = pos.0 / self.plot_size.0;
		let plot_idy = pos.1 / self.plot_size.1;
		for x in plot_idx*self.plot_size.0 .. (plot_idx+1)*self.plot_size.0 {
			for y in plot_idy*self.plot_size.1 .. (plot_idy+1)*self.plot_size.1 {
				positions.push(Pos(x, y));
			}
		}
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
	
	fn change_tile(&mut self, source_pos: Pos, from: Option<Entity>, to: Option<Entity>) -> bool {
		for pos in self.tiles_in_plot(source_pos) {
			if let from = self.get(pos) {
				self.set(pos, to);
				return true;
			}
		}
		return false;
	}
	
	fn change_stockpile(&mut self, pos: Pos, from: Option<Resource>, to: Option<Resource>) -> bool {
		self.change_tile(pos, Some(Entity::Stockpile(from)), Some(Entity::Stockpile(to)))
	}
	
	pub fn add_resource(&mut self, pos: Pos, res: Resource) -> bool {
		self.change_stockpile(pos, None, Some(res))
	}
	
	pub fn take_resource(&mut self, pos: Pos, res: Resource) -> bool {
		self.change_stockpile(pos, Some(res), None)
	}
	
	pub fn neighbour_lane(&mut self, pos: Pos, dir: Direction) -> Vec<Pos> {
		let lane = Vec::new();
		// todo: search correct positions
		lane
	}
}
