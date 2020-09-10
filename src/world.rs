
use std::collections::HashMap;

use crate::{
	field::Field,
	commands::{Command, Action},
	UserId,
	entity::Entity,
	resources::{Resource, ResourceCount},
	buildings::BuildingType,
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

	pub fn new(field: Field) -> World{
		Self {field}
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
		let ordered = Self::order_commands(commands);
		for (user, command) in ordered.iter() {
			let data = user_data.entry(user.clone()).or_insert_with(UserData::new);
			self.run_command(user, command, data);
		}
		
	}
	
	pub fn run_command(&mut self, user: &UserId, command: &Command, user_data: &mut UserData) {
		if user_data.ap_left <= 0 {
			return
		}
		user_data.ap_left -= 1;
		
		if command.action == Action::Claim && user_data.keeps.is_empty() {
			if let Some(pos) = self.field.claim_first_keep(command.pos, user.clone()) {
				user_data.keeps.push(pos);
			}
		}
		
		if self.field.plot_owner(command.pos).as_ref() != Some(user) {
			return
		}
		
		match (command.action.clone(), self.field.get(command.pos)) {
			(Action::Build(building), None) => {
				let (cost, ent) = building.cost_result();
				if self.field.pay(command.pos, &cost){
					self.field.set_tile(command.pos, ent);
				} else if building == BuildingType::Woodcutter && user_data.has_woodcutter == false {
					// bootstrap the first woodcutter
					user_data.has_woodcutter = true;
				}
			}
			
			(Action::Move(target), Some(ent)) => {
				if self.field.get(target).is_some() {
					return;
				}
				match ent {
					Entity::Raider => {
						self.field.clear_tile(command.pos);
						self.field.set_tile(target, ent);
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
}
