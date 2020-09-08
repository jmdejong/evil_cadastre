use std::str::FromStr;

use crate::{
	playercommand::PlayerCommand,
	errors::ParseError,
	parse_err
};

pub fn parse_input(input: &str) -> Vec<Result<PlayerCommand, ParseError>> {
	let commands = input.split("\n").into_iter().filter_map(|command_line| {
		let command_text = command_line.trim();
		if command_text.is_empty() {
			return None
		}
		let command = PlayerCommand::from_str(command_text);
		Some(command.map_err(|e| parse_err!("Failed to parse '{}': {}", command_text, e.msg)))
	}).collect();
	commands
}
