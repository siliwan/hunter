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

    match &args[1][..] {
	"start" => socket::start_socket(),
	"get" => get_handler(args),
	"add" => add(),
	"delete" => delete(),
	_ => println!("sajkh"),
    };
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
