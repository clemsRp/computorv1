
use std::env;

mod fraction;
mod lexer;
mod polynomial;
mod parser;
mod equation;

use equation::Equation;

fn main() {
	// Get input
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		return
	}

	let equation = Equation::new(args[1].clone()).unwrap();
	equation.print();
}
