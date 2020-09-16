
use std::collections::HashSet;

use crate::{
	field::Field,
	UserId,
	entity::Entity,
	resources::{Resource, ResourceCount},
	Pos,
	locations::Direction
};


pub fn claim_first_keep(field: &mut Field, source_pos: Pos, userid: UserId) -> Option<Pos> {
	if !field.is_valid(source_pos){
		return None;
	}
	let pos = field.keep_location(source_pos);
	if field.get(pos).is_some() {
		return None
	}
	for dir in Direction::directions() {
		if let Some(Entity::Keep(_)) = field.get(field.keep_location(pos + dir.to_pos() * field.plot_size)) {
			return None
		}
	}
	field.set_tile(pos, Entity::Capital(userid));
	Some(pos)
}

pub fn pay(field: &mut Field, pos: Pos, cost: &ResourceCount) -> bool {
	let mut available_resources = ResourceCount::default();
	for pos in field.tiles_in_plot(pos){
		if let Some(Entity::Stockpile(Some(res))) = field.get(pos) {
			available_resources.add_resource(res);
		}
	}
	if available_resources.can_afford(cost) {
		for res in cost.to_vec() {
			field.change_tile(pos, Some(Entity::Stockpile(Some(res))), Some(Entity::Stockpile(None)));
		}
		return true;
	}
	false
}


pub fn move_unit_destination(field: &Field, from: Pos, to: Pos) -> Option<Pos> {
	if field.keep_location(from) != field.keep_location(to) {
		return None;
	}
	match field.get(to) {
		Some(Entity::Road) => field.cross_pos(to),
		Some(_) => None,
		None => Some(to)
	}
}

pub fn move_resource_destination(field: &Field, from: Pos, to: Pos) -> Option<Pos> {
	if field.keep_location(from) != field.keep_location(to) {
		return None;
	}
	match field.get(to) {
		Some(Entity::Tradepost) => field.cross_pos(to),
		Some(Entity::Stockpile(None)) => Some(to),
		_ => None
	}
}

pub fn add_resource(field: &mut Field, pos: Pos, res: Resource) -> Option<Pos> {
	field.change_tile(pos, Some(Entity::Stockpile(None)), Some(Entity::Stockpile(Some(res))))
}

pub fn destroy_keep(field: &mut Field, pos: Pos) -> Option<()> {
	let user: UserId = field.plot_owner(pos)?;
	field.clear_tile(field.keep_location(pos));
	for dir in Direction::directions(){
		let neighbour = dir.to_pos() * field.plot_size + pos;
		if field.plot_owner(neighbour).as_ref() != Some(&user) {
			continue;
		}
		let mut has_capital = false;
		let mut lands: HashSet<Pos> = HashSet::new();
		let mut fringe: Vec<Pos> = Vec::new();
		fringe.push(neighbour);
		while let Some(keep) = fringe.pop().map(|p|field.keep_location(p)) {
			if field.plot_owner(keep).as_ref() != Some(&user) || lands.contains(&keep){
				continue;
			}
			if field.get(keep) == Some(Entity::Capital(user.clone())) {
				has_capital = true;
			}
			lands.insert(keep);
		}
		if !has_capital {
			for keep in lands {
				field.clear_tile(keep);
			}
		}
	}
	Some(())
}
