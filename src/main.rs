
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

	// Create equation
	let mut equation = Equation::new(args[1].clone()).expect("vntgmlrdp");
	equation = equation.reduced();
	let degree = equation.get_degree();

	println!("Reduced form: {}", equation);
	println!("Polynomial degree: {}", degree);

	let (prefix, solutions) = equation.solve();
	println!("{}", prefix);

	for solution in solutions.iter() {
		println!("{}", solution);
	}
}
