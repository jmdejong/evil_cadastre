


extern crate strum;
extern crate strum_macros;
extern crate structopt;
extern crate chrono;
pub mod input;
pub mod locations;
pub mod commands;
pub mod user;
mod utils;
mod errors;
pub mod parser;
mod entity;
pub mod field;
pub mod world;
mod buildings;
mod resources;
mod rules;

use crate::{
	locations::{Pos, Size},
	user::UserId,
	utils::{partition}
};
