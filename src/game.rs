

use std::path::PathBuf;

use crate::{
	input::{InputMethod, HomeScraper},
	UserId,
	commands::Command,
	parser,
	field::Field,
	world::World,
	Pos
};


pub fn read_all_commands(input: &HomeScraper) -> Vec<(UserId, Vec<Command>)>{
	let users = input.find_users().expect("Can not find user list");
	users.into_iter().filter_map(|(userid, connection)| {
		let command_text = input.read_input(&connection)?;
		let user_commands: Vec<Command> = parser::parse_input(&command_text).into_iter().filter_map(|res| 
			res.map_err(|pe| {
				let _ = input.output(&connection, &format!("Parse Error: {:?}", pe));
				pe
			}).ok()
		).collect();
		Some((userid, user_commands))
	}).collect()
}

pub fn main(){
	let input = HomeScraper {
		user_dir: PathBuf::from("/home/"),
		game_dir: PathBuf::from(".cadastre/evil/"),
		command_fname: PathBuf::from("commands"),
		log_fname: PathBuf::from("log.log")
	};
	let all_commands = read_all_commands(&input);
	for (user, user_commands) in all_commands.iter() {
		println!("    {}:", user.0);
		for command in user_commands {
			println!("{:?}", command);
		}
	}
	let mut world = World::new(Field::new(Pos(10, 10), Pos(5, 5)));
	world.update(&all_commands);
}
