use std::env::*;

mod socket;
mod init;
mod parse;

/// The main function of hunter.
/// # What it does
/// It collects the arguments and if one of the arguments matches,
/// it will call the appropriate function.
fn main() {
    let args: Vec<String> = args().collect();

    println!("{:?}", args);
    for e in args.clone().into_iter() {
	let _return = match e.as_str() {
	    "--socket=true" => socket::start_socket(),
	    "--socket=false" => init::start_no_socket(args.clone()),
	    _ => {Ok(())},
	};
    }
}
