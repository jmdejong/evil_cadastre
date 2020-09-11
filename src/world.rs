
use std::collections::{HashMap, HashSet};

use crate::{
	field::Field,
	commands::{Command, Action},
	UserId,
	entity::Entity,
	resources::{Resource, ResourceCount},
	buildings::BuildingType,
	utils,
	Pos
};

pub struct World {
	field: Field
}

#[derive(Debug, Clone)]
pub struct UserData {
	pub keeps: Vec<Pos>,
	pub has_woodcutter: bool,
	pub ap_left: i32
}

impl UserData {
	pub fn new() -> Self {
		Self {
			keeps: Vec::new(),
			has_woodcutter: false,
			ap_left: 10
		}
	}
}

impl World {
	
	pub fn init(plot_size: Pos, size: Pos) -> World {
		let mut field = Field::new(plot_size, size);
		for keep in field.list_keeps() {
			let plot = keep / plot_size;
			let plot_start = plot * plot_size;
			// corners are unavailable
			field.set_tile(plot_start, Entity::Swamp);
			field.set_tile(plot_start + Pos::new(plot_size.x-1, 0), Entity::Swamp);
			field.set_tile(plot_start + Pos::new(0, plot_size.y-1), Entity::Swamp);
			field.set_tile(plot_start + Pos::new(plot_size.x-1, plot_size.y-1), Entity::Swamp);
			// Place some random forests and swamps
			let tiles: Vec<Pos> = field.find_all(keep, None);
			let r0 = utils::randomize((plot.x + plot.y * size.y) as u32);
			let r1 = utils::randomize(r0);
			let r2 = utils::randomize(r1);
			field.set_tile(tiles[(r0 as usize) % tiles.len()], Entity::Forest);
			field.set_tile(tiles[(r1 as usize) % tiles.len()], Entity::Forest);
			field.set_tile(tiles[(r2 as usize) % tiles.len()], Entity::Swamp);
		}
		Self::new(field)
	}
	
	pub fn new(field: Field) -> World{
		Self{field}
	}
	
	fn calculate_user_data(&self) -> HashMap<UserId, UserData> {
		let mut data = HashMap::new();
		for pos in self.field.list_keeps() {
			if let Some(Entity::Keep(userid)) = self.field.get(pos) {
				let mut entry = data.entry(userid.clone()).or_insert_with(UserData::new);
				entry.keeps.push(pos);
				for tile in self.field.tiles_in_plot(pos){
					if self.field.get(tile) == Some(Entity::Woodcutter){
						entry.has_woodcutter = true;
					}
				}
			}
		}
		data
	}
	
	fn order_commands(commands: &[(UserId, Vec<Command>)]) -> Vec<(UserId, Command)> {
		let mut build_commands = Vec::new();
		let mut move_commands = Vec::new();
		let mut attack_commands = Vec::new();
		let mut remove_commands = Vec::new();
		let mut use_commands = Vec::new();
		let mut claim_commands = Vec::new();
		for (user, command_list) in commands {
			for command in command_list {
				match command.action {
					Action::Build(_) => {
						build_commands.push((user, command));
					}
					Action::Move(_) => {
						move_commands.push((user, command));
					}
					Action::Attack(_) => {
						attack_commands.push((user, command));
					}
					Action::Use => {
						use_commands.push((user, command));
					}
					Action::Remove => {
						remove_commands.push((user, command));
					}
					Action::Claim => {
						claim_commands.push((user, command));
					}
				}
			}
		}
		let mut ordered_commands = Vec::new();
		ordered_commands.append(&mut claim_commands);
		ordered_commands.append(&mut attack_commands);
		ordered_commands.append(&mut use_commands);
		ordered_commands.append(&mut build_commands);
		ordered_commands.append(&mut move_commands);
		ordered_commands.append(&mut remove_commands);
		
		ordered_commands.into_iter().map(|(u, c)|(u.clone(), c.clone())).collect()
	}
	
	pub fn update(&mut self, commands: &[(UserId, Vec<Command>)]){
		let mut user_data = self.calculate_user_data();
		let mut used_tiles = HashSet::new();
		let ordered = Self::order_commands(&commands.iter().map(|(user, commands)| {
			let data = user_data.entry(user.clone()).or_insert_with(UserData::new);
			(user.clone(), utils::truncated(commands, data.ap_left as usize))
		}).collect::<Vec<(UserId, Vec<Command>)>>());
		for (user, command) in ordered.iter() {
			// todo: make sure no 2 commands can run on the same unit/building in one update
			let data = user_data.entry(user.clone()).or_insert_with(UserData::new);
			self.run_command(user, command, data, &mut used_tiles);
		}
		
	}
	
	fn move_destination(&self, from: Pos, to: Pos) -> Option<Pos> {
		if self.field.keep_location(from) != self.field.keep_location(to) {
			return None;
		}
		match self.field.get(to) {
			Some(Entity::Road) => self.field.cross_pos(to),
			Some(_) => None,
			None => Some(to)
		}
	}
	
	pub fn run_command(&mut self, user: &UserId, command: &Command, user_data: &mut UserData, used_tiles: &mut HashSet<Pos>) {
		
		if used_tiles.contains(&command.pos){
			return;
		}
		
		if command.action == Action::Claim && user_data.keeps.is_empty() {
			if let Some(pos) = self.field.claim_first_keep(command.pos, user.clone()) {
				user_data.keeps.push(pos);
				used_tiles.insert(command.pos);
			}
		}
		
		if self.field.plot_owner(command.pos).as_ref() != Some(user) {
			return
		}
		
		used_tiles.insert(command.pos);
		
		match (command.action.clone(), self.field.get(command.pos)) {
			(Action::Build(building), None) => {
				if building == BuildingType::Road && self.field.across_border(command.pos) == None {
					return;
				}
				if building == BuildingType::Woodcutter && !self.field.neighbours(command.pos, Some(Entity::Forest)){
					return
				}
				let (cost, ent) = building.cost_result();
				if self.field.pay(command.pos, &cost){
					self.field.set_tile(command.pos, ent);
				}
			}
			
			(Action::Move(target), Some(ent)) => {
				if used_tiles.contains(&target) {
					return;
				}
				if let Some(pos) = self.move_destination(command.pos, target) {
					match ent {
						Entity::Raider => {
							self.field.clear_tile(command.pos);
							self.field.set_tile(pos, ent);
							used_tiles.insert(pos);
							used_tiles.insert(target);
						}
						_ => {}
					}
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
								if props.destructible {
									self.field.clear_tile(pos);
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
						self.field.add_resource(command.pos, Resource::Wood);
					}
					Entity::Farm => {
						self.field.add_resource(command.pos, Resource::Food);
					}
					Entity::Lair => {
						if self.field.pay(command.pos, &ResourceCount::from_vec(&vec![Resource::Food, Resource::Food, Resource::Food])) {
							self.field.change_tile(command.pos, None, Some(Entity::Raider));
						}
					}
					_ => ()
				}
			}
			
			(Action::Remove, Some(ent)) => {
				if ent.properties().removable {
					self.field.clear_tile(command.pos);
				}
			}
			_ => {}
		}
	}
	
	pub fn serialise(&self) -> String {
		self.field.to_string()
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;
	
	macro_rules! tileis {
			($world: expr, $x: expr, $y: expr, $val: expr) => {assert_eq!($world.field.get(Pos::new($x, $y)), $val)}
	}
	
	fn parse_commands(u: &str, c: &[&str]) -> (UserId, Vec<Command>) {
		(UserId(u.to_string()), c.iter().map(|s| Command::from_str(s).unwrap()).collect())
	}
	
	#[test]
	fn test_simple_commands() {
		let mut world = World {field: Field::from_str("size:5,5 plot_size:10,10 /").unwrap()};
		let (user, commands) = parse_commands("user", &[
			"2,1 build stockpile",
			"15,2 build woodcutter",
			"6,2 build woodcutter",
			"6,3 build woodcutter",
			"0,0 claim",
			"11,1 claim",
			"11,2 build stockpile",
			"6,2 build stockpile",
			"8,0 build stockpile",
			"8,1 build stockpile",
			"8,2 build stockpile",
			"8,3 build stockpile",
			"8,4 build stockpile",
			"8,5 build stockpile"
		]);
		world.update(&vec![(user.clone(), commands)]);
		assert_eq!(world.field.plot_owner(Pos::new(0,0)), Some(user.clone()));
		assert_eq!(world.field.plot_owner(Pos::new(9,9)), Some(user.clone()));
		assert_eq!(world.field.plot_owner(Pos::new(11,11)), None);
		assert_eq!(world.field.plot_owner(Pos::new(1,11)), None);
		assert_eq!(world.field.plot_owner(Pos::new(11,1)), None);
		tileis!(world, 2,1, Some(Entity::Stockpile(None)));
		tileis!(world, 15,2, None);
		tileis!(world, 6,2, None);
		tileis!(world, 6,3, None);
		tileis!(world, 11,2, None);
		tileis!(world, 8,0, Some(Entity::Stockpile(None)));
		tileis!(world, 8,1, Some(Entity::Stockpile(None)));
		tileis!(world, 8,2, None);
		assert_eq!(world.field, Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			2,1 stockpile;
			8,0 stockpile;
			8,1 stockpile;"
		).unwrap());
	}
	
	#[test]
	fn test_woodcutting(){
		let mut world = World {field: Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			0,5 woodcutter;
			1,5 stockpile;
			2,5 stockpile;
			0,2 stockpile;
			9,5 woodcutter;
			10,5 stockpile;"
		).unwrap()};
		let (user, commands) = parse_commands("user", &[
			"0,5 use",
			"9,5 use"
		]);
		world.update(&vec![(user, commands)]);
		
		assert_eq!(world.field, Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			0,5 woodcutter;
			1,5 stockpile:wood;
			2,5 stockpile:wood;
			0,2 stockpile;
			9,5 woodcutter;
			10,5 stockpile;"
		).unwrap());
	}
	
	#[test]
	fn test_attack(){
		let mut world = World {field: Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			6,6 lair;
			1,9 raider;
			3,3 woodcutter;
			3,7 raider;
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			1,13 farm;
			3,17 raider;
			3,16 farm;"
		).unwrap()};
		world.update(&vec![
			parse_commands("user", &[
				"1,9 attack south",
				"11,6 attack west"
			]),
			parse_commands("other", &[
				"3,17 attack north",
			])
		]);
		
		assert_eq!(world.field, Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			6,6 lair;
			1,9 raider;
			3,3 woodcutter;
			3,7 raider;
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			3,17 raider;
			3,16 farm;"
		).unwrap());
	}
	
	
	#[test]
	fn test_move(){
		let mut world = World {field: Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			1,1 raider;
			1,2 raider;
			1,3 raider;
			1,4 raider;
			1,5 raider;
			1,6 raider;
			1,7 raider;
			1,8 raider;
			1,9 raider;
			2,1 raider;
			7,7 stockpile;
			9,9 road;
			6,6 road;
			2,9 road;
			9,2 road;
			0,1 road;
			1,0 road;
			
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			1,13 farm;
			3,17 raider;
			3,16 farm;"
		).unwrap()};
		world.update(&vec![
			parse_commands("user", &[
				"1,1 move 0,0",
				"1,2 move 0,0",
				"1,3 move 7,7",
				"1,4 move 5,5",
				
				"1,5 move 6,6",
				"1,6 move 9,9",
				"1,7 move 9,2",
				"1,8 move 2,9",
				"1,0 move 1,0",
				"2,1 move 19,9",
			]),
		]);
		assert_eq!(world.field, Field::from_str(
			"size:5,5 plot_size:10,10 /
			5,5 keep:user;
			0,0 raider;
			1,2 raider;
			1,3 raider;
			1,4 raider;
			1,5 raider;
			1,6 raider;
			10,2 raider;
			1,8 raider;
			1,9 raider;
			2,1 raider;
			7,7 stockpile;
			9,9 road;
			6,6 road;
			2,9 road;
			9,2 road;
			0,1 road;
			1,0 road;
			
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			1,13 farm;
			3,17 raider;
			3,16 farm;"
		).unwrap());
	}
}
