
mod input;
mod locations;
mod commands;
mod user;
mod game;
mod utils;
mod errors;
mod parser;
mod entity;
mod field;
mod world;
mod buildings;
mod resources;

use crate::{
	locations::{Pos, Size},
	user::UserId,
	utils::{partition}
};

fn main() {
	game::main();
}
