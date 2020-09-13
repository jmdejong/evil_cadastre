
extern crate strum;
extern crate strum_macros;
extern crate structopt;
extern crate chrono;
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
mod rules;

use crate::{
	locations::{Pos, Size},
	user::UserId,
	utils::{partition}
};

fn main() {
	game::main();
}
