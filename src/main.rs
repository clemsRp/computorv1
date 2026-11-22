
mod fraction;
mod lexer;

use std::env;
use lexer::tokenize;

fn main() {
	// Get input
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		return
	}

	let tokens = tokenize(args[1].clone());

	for i in 0..tokens.len() {
		println!("kind: {:?}, text: '{}'", tokens[i].get_kind(), tokens[i].get_text());
	}
}
