

use std::path::PathBuf;
use std::io::Read;
use std::str::FromStr;

extern crate structopt;
use structopt::StructOpt;

use crate::{
	input::{InputMethod, HomeScraper},
	UserId,
	commands::Command,
	parser,
	field::Field,
	world::World,
	Pos
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

