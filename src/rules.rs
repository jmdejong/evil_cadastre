


use crate::{
	field::Field,
	UserId,
	entity::Entity,
	resources::{Resource, ResourceCount},
	Pos
};


pub fn claim_first_keep(field: &mut Field, source_pos: Pos, userid: UserId) -> Option<Pos> {
	if !field.is_valid(source_pos){
		return None;
	}
	let pos = field.keep_location(source_pos);
	match field.get(pos) {
		Some(Entity::Keep(_)) => None,
		Some(_) => {panic!("plot without keep: {:?}", pos)}
		None => {
			field.set_tile(pos, Entity::Keep(userid));
			Some(pos)
		}
	}
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
