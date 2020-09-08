
use std::path::PathBuf;
use std::fs;
// use std::env;
use std::io;
use std::io::Write;
use std::str::FromStr;
use chrono::Utc;

use crate::{
	UserId,
	playercommand::PlayerCommand
};

pub trait InputMethod {
	type IO;
	type Err;
	
	fn find_users(&self) -> Result<Vec<(UserId, Self::IO)>, Self::Err>;
	fn output(&self, connection: &Self::IO, text: &str) -> Result<(), Self::Err>;
	fn read_input(&self, connection: &Self::IO) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct HomeScraper {
	pub user_dir: PathBuf,
	pub game_dir: PathBuf,
	pub command_fname: PathBuf,
	pub log_fname: PathBuf
}

impl HomeScraper {
	
	
	pub fn read_commands(&self, home_dir: &PathBuf) -> Option<Vec<PlayerCommand>> {
		let commands_string = self.read_input(home_dir)?;
		let commands = commands_string.split("\n").into_iter().filter_map(|command_line| {
			let command_text = command_line.trim();
			if command_text.is_empty() {
				return None
			}
			match PlayerCommand::from_str(command_text){
				Ok(command) => Some(command),
				Err(_) => {
					let _ = self.output(home_dir, &format!("Parse error parsing '{}'", command_line));
					None
				}
			}
		}).collect();
		Some(commands)
	}
	
	pub fn read_all_commands(&self) -> io::Result<Vec<(UserId, Vec<PlayerCommand>)>> {
		Ok(self.find_users()?.into_iter().filter_map(|(user, path)| Some((user, self.read_commands(&path)?))).collect())
	}
	
}

impl InputMethod for HomeScraper {
	type Err = io::Error;
	type IO = PathBuf;

	fn find_users(&self) -> io::Result<Vec<(UserId, PathBuf)>> {
		Ok(fs::read_dir(&self.user_dir)?
			.filter_map(|dir_entry| {
				// Only take the users that have an accessible game directory
				// If any of these functions errors for some other reason there's not really something we can do
				// just ignore this user
				let dir = dir_entry.ok()?;
				let name = dir.file_name().into_string().ok()?;
				let path = dir.path().join(&self.game_dir);
				let _ = fs::read_dir(&path).ok()?; // filter out users that don't have the directory
				Some((UserId(name), path))
			}).collect()
		)
	}
	
	fn read_input(&self, home_dir: &PathBuf) -> Option<String> {
		let path: PathBuf = home_dir.join(&self.command_fname);
		fs::read_to_string(&path).map_err(|err| {
			let _ = self.output(home_dir, &format!("File error loading {:?}: {}", &path, err.to_string()));
			err
		}).ok()
	}
	
	fn output(&self, home_dir: &PathBuf, msg: &str) -> io::Result<()> {
		let mut file = fs::OpenOptions::new().append(true).create(true).open(home_dir.join(&self.log_fname))?;
		writeln!(file, "{}  {}", Utc::now(), msg)
	}
}