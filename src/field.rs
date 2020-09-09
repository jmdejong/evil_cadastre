
use std::collections::HashMap;

use crate::{
	Pos,
	Size,
	locations::Direction,
	entity::Entity,
	resources::{Resource, ResourceCount},
	UserId
};

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
		let base = plot * self.plot_size + self.plot_size / 2;
		let offset = (Pos(1, 1) - self.plot_size % 2) * plot % 2;
		base - offset
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
		let plot = pos / self.plot_size;
		for x in plot.0*self.plot_size.0 .. (plot.0+1)*self.plot_size.0 {
			for y in plot.1*self.plot_size.1 .. (plot.1+1)*self.plot_size.1 {
				positions.push(Pos(x, y));
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
	
	fn change_stockpile(&mut self, pos: Pos, from: Option<Resource>, to: Option<Resource>) -> bool {
		self.change_tile(pos, Some(Entity::Stockpile(from)), Some(Entity::Stockpile(to)))
	}
	
	pub fn add_resource(&mut self, pos: Pos, res: Resource) -> bool {
		self.change_stockpile(pos, None, Some(res))
	}
	
	pub fn take_resource(&mut self, pos: Pos, res: Resource) -> bool {
		self.change_stockpile(pos, Some(res), None)
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
				self.take_resource(pos, res);
			}
			return true;
		}
		false
	}
}
