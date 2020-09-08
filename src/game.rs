

use std::path::PathBuf;

use crate::{
	input::{InputMethod, HomeScraper},
	UserId,
	playercommand::PlayerCommand,
	parser
};


pub fn read_all_commands(input: &HomeScraper) -> Vec<(UserId, Vec<PlayerCommand>)>{
	let users = input.find_users().expect("Can not find user list");
	users.into_iter().filter_map(|(userid, connection)| {
		let command_text = input.read_input(&connection)?;
		let user_commands: Vec<PlayerCommand> = parser::parse_input(&command_text).into_iter().filter_map(|res| 
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
	for (user, user_commands) in all_commands {
		println!("    {}:", user.0);
		for command in user_commands {
			println!("{:?}", command);
		}
	}
}
