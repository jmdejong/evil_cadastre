
use std::path::PathBuf;
use std::fs;
// use std::env;
use std::io;
use std::io::Write;
use chrono::Utc;

use crate::{
	UserId,
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
	pub command_fnames: Vec<PathBuf>,
	pub log_fname: PathBuf
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
		for command_fname in self.command_fnames.iter() {
			let path: PathBuf = home_dir.join(&command_fname);
			let res = fs::read_to_string(&path).map_err(|err| {
				let _ = self.output(home_dir, &format!("File error loading {:?}: {}", &path, err.to_string()));
				err
			});
			if let Ok(command_s) = res {
				return Some(command_s);
			}
		}
		None
	}
	
	fn output(&self, home_dir: &PathBuf, msg: &str) -> io::Result<()> {
		let mut file = fs::OpenOptions::new().append(true).create(true).open(home_dir.join(&self.log_fname))?;
		writeln!(file, "{}  {}", Utc::now(), msg)
	}
}
