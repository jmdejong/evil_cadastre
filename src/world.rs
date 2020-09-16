
use std::collections::{HashSet};

use crate::{
	field::Field,
	commands::{Command, Action},
	UserId,
	entity::Entity,
	resources::{Resource, ResourceCount},
	buildings::BuildingType,
	rules,
	utils,
	Pos
};

pub struct World {
	pub field: Field
}


impl World {
	
	pub fn init(plot_size: Pos, size: Pos) -> World {
		let mut field = Field::new(plot_size, size);
		for keep in field.list_keeps() {
			let plot = keep / plot_size;
			let plot_start = plot * plot_size;
			// corners are unavailable
			field.set_tile(plot_start, Entity::Rock);
			field.set_tile(plot_start + Pos::new(plot_size.x-1, 0), Entity::Rock);
			field.set_tile(plot_start + Pos::new(0, plot_size.y-1), Entity::Rock);
			field.set_tile(plot_start + Pos::new(plot_size.x-1, plot_size.y-1), Entity::Rock);
			// Place some random forests and swamps
			let tiles: Vec<Pos> = field.find_all(keep, None);
			let r0 = utils::randomize((plot.x + plot.y * 67679) as u32);
			let r1 = utils::randomize(r0);
			let r2 = utils::randomize(r1);
			field.set_tile(tiles[(r0 as usize) % tiles.len()], Entity::Forest);
			field.set_tile(tiles[(r2 as usize) % tiles.len()], Entity::Swamp);
			field.set_tile(tiles[(r1 as usize) % tiles.len()], Entity::Forest);
		}
		Self::new(field)
	}
	
	pub fn new(field: Field) -> World{
		Self{field}
	}
	
	fn order_commands(commands: &[(UserId, Vec<Command>)]) -> Vec<Vec<(UserId, Command)>> {
		let mut command_iterators = Vec::new();
		for (user, comms) in commands {
			command_iterators.push((user.clone(), comms.iter()));
		}
		let mut ordered_commands = Vec::new();
		loop {
			let heads: Vec<(UserId, Command)> = command_iterators
				.iter_mut()
				.filter_map(|(user, it)| Some((user.clone(), it.next()?)))
				.map(|(user, command)| (user, command.clone()))
				.collect();
			if heads.is_empty(){
				break;
			}
			ordered_commands.push(heads);
		}
		ordered_commands
	}
	
	pub fn update(&mut self, commands: &[(UserId, Vec<Command>)]){
		let mut used_tiles = HashSet::new();
		let ordered = Self::order_commands(&commands.iter().map(|(user, commands)| {
			(user.clone(), utils::truncated(commands, 10))
		}).collect::<Vec<(UserId, Vec<Command>)>>());
		for command_round in ordered {
			let mut destroyed = Vec::new();
			for (user, command) in command_round {
				self.run_command(&user, &command, &mut used_tiles, &mut destroyed);
			}
			for tile in destroyed {
				self.field.clear_tile(tile);
			}
		}
	}
	
	pub fn run_command(&mut self, user: &UserId, command: &Command, used_tiles: &mut HashSet<Pos>, destroyed: &mut Vec<Pos>) {
		
		if used_tiles.contains(&command.pos){
			return;
		}
		
		if command.action == Action::Claim && !self.field.list_keeps().iter().any(|p| self.field.get(*p) == Some(Entity::Capital(user.clone()))) {
			rules::claim_first_keep(&mut self.field, command.pos, user.clone());
		}
		
		if self.field.plot_owner(command.pos).as_ref() != Some(user) {
			return
		}
		
		used_tiles.insert(command.pos);
		
		match (command.action.clone(), self.field.get(command.pos)) {
			(Action::Build(building), None) => {
				if (
						building == BuildingType::Road ||
						building == BuildingType::Tradepost ||
						building == BuildingType::Scoutpost
						) && self.field.across_border(command.pos) == None {
					return;
				}
				if building == BuildingType::Woodcutter && !self.field.neighbours(command.pos, Some(Entity::Forest)){
					return;
				}
				if building == BuildingType::Quarry && !self.field.neighbours(command.pos, Some(Entity::Rock)){
					return;
				}
				let (cost, ent) = building.cost_result();
				if rules::pay(&mut self.field, command.pos, &cost){
					self.field.set_tile(command.pos, ent);
				}
			}
			
			(Action::Move(target), Some(ent)) => {
				if used_tiles.contains(&target) {
					return;
				}
				match ent {
					e if e.properties().movable => {
						if let Some(pos) = rules::move_unit_destination(&self.field, command.pos, target) {
							self.field.switch_tiles(command.pos, pos);
							used_tiles.insert(pos);
							used_tiles.insert(target);
						}
					}
					Entity::Stockpile(Some(_res)) => {
						if let Some(pos) = rules::move_resource_destination(&self.field, command.pos, target) {
							self.field.switch_tiles(command.pos, pos);
							used_tiles.insert(pos);
							used_tiles.insert(target);
						}
					}
					Entity::Capital(owner) if &owner == user => {
						if self.field.get(target) == Some(Entity::Keep(user.clone())) {
							self.field.switch_tiles(command.pos, target);
							used_tiles.insert(target);
						}
					}
					_ => {}
				}
			}
			
			(Action::Attack(dir), Some(ent)) => {
			
				let lane = self.field.neighbour_lane(command.pos, dir);
				if lane.is_empty() || self.field.plot_owner(lane[0]).as_ref() == Some(user){
					return;
				}
				match ent {
					Entity::Raider => {
						for pos in lane {
							if let Some(target) = self.field.get(pos) {
								let props = target.properties();
								if props.destructible && !props.strong {
									destroyed.push(pos);
								}
								if props.stopping {
									break;
								}
							}
						}
					}
					Entity::Warrior => {
						for pos in lane {
							if let Some(target) = self.field.get(pos) {
								let props = target.properties();
								if props.mortal {
									destroyed.push(pos);
									break;
								}
							}
						}
					}
					Entity::Ram => {
						for pos in lane {
							if let Some(target) = self.field.get(pos) {
								let props = target.properties();
								if props.destructible {
									destroyed.push(pos);
								}
								if props.stopping {
									break;
								}
							}
						}
					}
					_ => {}
				}
			}
			
			(Action::Use, Some(ent)) => {
				match ent {
					Entity::Woodcutter => {
						rules::add_resource(&mut self.field, command.pos, Resource::Wood);
					}
					Entity::Quarry => {
						rules::add_resource(&mut self.field, command.pos, Resource::Wood);
					}
					Entity::Farm => {
						rules::add_resource(&mut self.field, command.pos, Resource::Food);
					}
					Entity::Lair => {
						if rules::pay(&mut self.field, command.pos, &ResourceCount::from_vec(&[Resource::Food, Resource::Food, Resource::Food])) {
							if let Some(pos) = self.field.change_tile(command.pos, None, Some(Entity::Raider)) {
								used_tiles.insert(pos);
							}
						}
					}
					Entity::Barracks => {
						if rules::pay(&mut self.field, command.pos, &ResourceCount::from_vec(&[Resource::Food, Resource::Food, Resource::Food, Resource::Food, Resource::Food, Resource::Wood, Resource::Stone])) {
							// todo: will require iron later
							if let Some(pos) = self.field.change_tile(command.pos, None, Some(Entity::Warrior)) {
								used_tiles.insert(pos);
							}
						}
					}
					Entity::Scoutpost => {
						if let Some(pos) = self.field.across_border(command.pos) {
							if self.field.plot_owner(command.pos) == self.field.plot_owner(pos) {
								return;
							}
							if self.field.tiles_in_plot(pos).into_iter().filter_map(|p| self.field.get(p)).any(|ent| ent.properties().defender) {
								return;
							}
							if rules::pay(&mut self.field, command.pos, &ResourceCount::from_vec(&[
									Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood,
									Resource::Food, Resource::Food, Resource::Food, Resource::Food, Resource::Food,
									Resource::Stone, Resource::Stone, Resource::Stone, Resource::Stone, Resource::Stone])) {
								let keep = self.field.keep_location(pos);
								rules::destroy_keep(&mut self.field, keep);
								self.field.set_tile(self.field.keep_location(pos), Entity::Keep(user.clone()));
							}
						}
					}
					_ => {}
				}
			}
			
			(Action::Remove, Some(ent)) if ent.properties().removable => {
				self.field.clear_tile(command.pos);
			}
			_ => {}
		}
	}
	
	pub fn serialise(&self) -> String {
		self.field.to_string()
	}
	
}

