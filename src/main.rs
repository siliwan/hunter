use std::env::*;

mod socket;
mod init;
mod parse;
mod get;

/// The main function of hunter.
/// # What it does
/// It collects the arguments and if one of the arguments matches,
/// it will call the appropriate function.
fn main() {
    let args: Vec<String> = args().collect();

	let first_arg = match get_first_arg_non_parameter(&args) {
		Some(x) => x,
		None => ""
	};

    match first_arg {
	"start" => socket::start_socket(),
	"get" => get_handler(args),
	"add" => add(),
	"delete" => delete(),
	"" => println!("No parameter was provided!"),
	_ => println!("Unkown parameter \"{}\" at position one.", first_arg),
    };
}

fn get_first_arg_non_parameter(args: &Vec<String>) -> Option<&String> {
	//Skip first argument always indicating program called from console
	for (i, arg) in args.iter().enumerate().skip(1) {
		if &arg.chars().count() < &3 || (&arg.chars().count() >= &3 && &arg[0..2] != "--") {
			return Some(&args[i]);
		}
	}

	return None;
}

fn get_handler(args: Vec<String>) {
    let data = match &args[2][..] {
	"id" => get::user_by_id(args[3].clone()),
	"key" => get::user_by_key(args[3].clone()),
	"config" => get::config(),
	_ => std::process::exit(-3),
    };

    println!("{}", data);
}

fn add() {}

fn delete() {}
