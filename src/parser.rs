use std::str::FromStr;

use crate::{
	commands::Command,
	errors::ParseError,
	parse_err
};

pub fn parse_input(input: &str) -> Vec<Result<Command, ParseError>> {
	input.split('\n').filter_map(|command_line| {
		let command_text = command_line.trim();
		if command_text.is_empty() || command_text.starts_with('#') {
			return None
		}
		let command = Command::from_str(command_text);
		Some(command.map_err(|e| parse_err!("Failed to parse '{}': {}", command_text, e.msg)))
	}).collect()
}
