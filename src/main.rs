
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

	let mut equation = Equation::new(args[1].clone()).expect("vntgmlrdp");
	equation.left = equation.left.sub(&equation.right);
	equation.right = equation.right.sub(&equation.right);
	equation.print();
}
