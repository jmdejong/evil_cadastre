

use std::path::PathBuf;
use std::io::Read;
use std::str::FromStr;

use structopt::StructOpt;

use evilcadastre::{
	input::{InputMethod, HomeScraper},
	user::UserId,
	commands::Command,
	parser,
	field::Field,
	world::World,
	locations::Pos
};


#[derive(StructOpt)]
#[structopt(name = "Evil Cadastre", about = "A turn-based stragegy game")]
enum Arguments {
	Init(InitArgs),
	Update(UpdateArgs)
}

#[derive(StructOpt)]
#[structopt(about = "Create a new world")]
pub struct InitArgs {

	#[structopt(short, long, default_value = "10,10", help="the width and height of a plot")]
	plot_size: Pos,
	
	#[structopt(short="s", long, help="the width and height of the world, measured in number of plots")]
	world_size: Pos
}

#[derive(StructOpt)]
#[structopt(about = "Update the world one step with user commands read from user directories")]
pub struct UpdateArgs {

	#[structopt(long, default_value="/home/", help="The base directory where all user home directories are located")]
	home_dirs: String,
	
	#[structopt(short, long, default_value=".cadastre/evil/", help="The path to the directory where the game files are relative to the home directory")]
	game_dir: String,
	
	#[structopt(short, long, required(true), help="The name that identifies the commands for this world")]
	world_name: Vec<String>
	
}

pub fn main(){

	match Arguments::from_args() {
		Arguments::Init(init_args) => init(init_args),
		Arguments::Update(update_args) => update(update_args)
	}
}

pub fn init(args: InitArgs){
	let world = World::init(args.plot_size, args.world_size);
	println!("{}", world.serialise());
}

pub fn update(args: UpdateArgs){
	let input = HomeScraper {
		user_dir: PathBuf::from(args.home_dirs),
		game_dir: PathBuf::from(args.game_dir),
		command_fnames: args.world_name.iter().map(PathBuf::from).collect(),
		log_fname: PathBuf::from(format!("{}.log", args.world_name[0]))
	};
	let all_commands = read_all_commands(&input);
	let mut world_s = String::new();
	std::io::stdin().read_to_string(&mut world_s).unwrap();
// 	let world_s = fs::read_to_string("world.evil").expect("failed to load world");
	let mut world = World::new(Field::from_str(&world_s).expect("Invalid world"));
	world.update(&all_commands);
	println!("{}", world.serialise());
}



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



#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;
	use crate::entity::Entity;
	
	macro_rules! tileis {
			($world: expr, $x: expr, $y: expr, $val: expr) => {assert_eq!($world.field.get(Pos::new($x, $y)), $val)}
	}
	
	fn parse_commands(u: &str, c: &[&str]) -> (UserId, Vec<Command>) {
		(UserId(u.to_string()), c.iter().map(|s| Command::from_str(s).unwrap()).collect())
	}
	
	#[test]
	fn test_simple_commands() {
		let mut world = World {field: Field::from_str("size:5,5; plot_size:10,10 ;;").unwrap()};
		let (user, commands) = parse_commands("user", &[
			"2,1 build stockpile",
			"15,2 build woodcutter",
			"6,2 build woodcutter",
			"6,3 build woodcutter",
			"0,0 claim",
			"11,1 claim",
			"11,2 build stockpile",
			"6,2 build stockpile",
			"8,0 build stockpile",
			"8,1 build stockpile",
			"8,2 build stockpile",
			"8,3 build stockpile",
			"8,4 build stockpile",
			"8,5 build stockpile"
		]);
		world.update(&vec![(user.clone(), commands)]);
		assert_eq!(world.field.plot_owner(Pos::new(0,0)), Some(user.clone()));
		assert_eq!(world.field.plot_owner(Pos::new(9,9)), Some(user.clone()));
		assert_eq!(world.field.plot_owner(Pos::new(11,11)), None);
		assert_eq!(world.field.plot_owner(Pos::new(1,11)), None);
		assert_eq!(world.field.plot_owner(Pos::new(11,1)), None);
		tileis!(world, 2,1, None);//Some(Entity::Stockpile(None)));
		tileis!(world, 15,2, None);
		tileis!(world, 6,2, Some(Entity::Stockpile(None)));
		tileis!(world, 6,3, None);
		tileis!(world, 11,2, None);
		tileis!(world, 8,0, Some(Entity::Stockpile(None)));
		tileis!(world, 8,1, Some(Entity::Stockpile(None)));
		tileis!(world, 8,2, None);
		assert_eq!(world.field, Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			6,2 stockpile;
			8,0 stockpile;
			8,1 stockpile;"
		).unwrap());
	}
	
	#[test]
	fn test_woodcutting(){
		let mut world = World {field: Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			0,5 woodcutter;
			1,5 stockpile;
			2,5 stockpile;
			0,2 stockpile;
			9,5 woodcutter;
			10,5 stockpile;"
		).unwrap()};
		let (user, commands) = parse_commands("user", &[
			"0,5 use",
			"9,5 use"
		]);
		world.update(&vec![(user, commands)]);
		
		assert_eq!(world.field, Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			0,5 woodcutter;
			1,5 stockpile:wood;
			2,5 stockpile:wood;
			0,2 stockpile;
			9,5 woodcutter;
			10,5 stockpile;"
		).unwrap());
	}
	
	#[test]
	fn test_attack(){
		let mut world = World {field: Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			6,6 lair;
			1,9 raider;
			3,3 woodcutter;
			3,7 raider;
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			1,13 farm;
			3,17 raider;
			3,16 farm;"
		).unwrap()};
		world.update(&vec![
			parse_commands("user", &[
				"1,9 attack south",
				"11,6 attack west"
			]),
			parse_commands("other", &[
				"3,17 attack north",
			])
		]);
		
		assert_eq!(world.field, Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			6,6 lair;
			1,9 raider;
			3,3 woodcutter;
			3,7 raider;
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			3,17 raider;
			3,16 farm;"
		).unwrap());
	}
	
	
	#[test]
	fn test_move(){
		let mut world = World {field: Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			1,1 raider;
			1,2 raider;
			1,3 raider;
			1,4 raider;
			1,5 raider;
			1,6 raider;
			1,7 raider;
			1,8 raider;
			1,9 raider;
			2,1 raider;
			7,7 stockpile;
			9,9 road;
			6,6 road;
			2,9 road;
			9,2 road;
			0,1 road;
			1,0 road;
			
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			1,13 farm;
			3,17 raider;
			3,16 farm;"
		).unwrap()};
		world.update(&vec![
			parse_commands("user", &[
				"1,1 move 0,0",
				"1,2 move 0,0",
				"1,3 move 7,7",
				"1,4 move 5,5",
				
				"1,5 move 6,6",
				"1,6 move 9,9",
				"1,7 move 9,2",
				"1,8 move 2,9",
				"1,0 move 1,0",
				"2,1 move 19,9",
			]),
		]);
		assert_eq!(world.field, Field::from_str(
			"size:5,5; plot_size:10,10 ;;
			5,5 keep:user;
			0,0 raider;
			1,2 raider;
			1,3 raider;
			1,4 raider;
			1,5 raider;
			1,6 raider;
			10,2 raider;
			1,8 raider;
			1,9 raider;
			2,1 raider;
			7,7 stockpile;
			9,9 road;
			6,6 road;
			2,9 road;
			9,2 road;
			0,1 road;
			1,0 road;
			
			
			15,4 keep:user;
			11,6 raider;
			
			4,15 keep:other;
			1,13 farm;
			3,17 raider;
			3,16 farm;"
		).unwrap());
	}
}

