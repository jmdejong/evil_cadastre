
use crate::{
	field::Field,
	commands::{Command, Action},
	UserId,
	entity::Entity,
	resources::{Resource, ResourceCount}
};

pub struct World {
	field: Field
}

impl World {

	pub fn new(field: Field) -> World{
		Self {field}
	}
	
	pub fn update(&mut self, commands: &[(UserId, Vec<Command>)]){
		let mut build_commands = Vec::new();
		let mut move_commands = Vec::new();
		let mut attack_commands = Vec::new();
		let mut remove_commands = Vec::new();
		let mut use_commands = Vec::new();
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
				}
			}
		}
		
		for (user, command) in attack_commands {
			self.run_command(user, command);
		}
		for (user, command) in use_commands {
			self.run_command(user, command);
		}
		for (user, command) in build_commands {
			self.run_command(user, command);
		}
		for (user, command) in move_commands {
			self.run_command(user, command);
		}
		for (user, command) in remove_commands {
			self.run_command(user, command);
		}
		
	}
	
	pub fn run_command(&mut self, user: &UserId, command: &Command) {
		
		if self.field.plot_owner(command.pos).as_ref() != Some(user) {
			return
		}
		
		match (command.action.clone(), self.field.get(command.pos)) {
			(Action::Build(building), None) => {
				let (cost, ent) = building.cost_result();
				if self.field.pay(command.pos, &cost) {
					self.field.set_tile(command.pos, ent);
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
