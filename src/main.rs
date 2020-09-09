
mod input;
mod locations;
mod playercommand;
mod user;
mod game;
mod utils;
mod errors;
mod parser;

use crate::{
	locations::{Pos, Direction},
	user::UserId,
	utils::{partition}
};

fn main() {
	game::main();
}
