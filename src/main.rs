
use std::env;

mod fraction;
mod lexer;
mod polynomial;
mod parser;
mod equation;

use equation::Equation;

fn main() {
	let mut step_active = false;

	// Get input
	let args: Vec<String> = env::args().collect();
	if args.len() == 3 {
		if args[2] != "--step" {
			return ;
		}
		step_active = true;
	} else if args.len() != 2 {
		return
	}

	let mut final_prints: Vec<String> = Vec::new();

	// Create equation
	let mut equation = Equation::new(args[1].clone()).expect("vntgmlrdp");
	final_prints.push(format!("[step] Raw form: {}", equation));

	// Simplified it
	let sub = equation.clone();
	let sub_equation =format!("{} - ({}) = {} - ({})", sub.get_left(), sub.get_right(), sub.get_right(), sub.get_right());
	final_prints.push(format!("[step] Subtract from both sides: {}", sub_equation));
	equation = equation.reduced();
	final_prints.push(format!("Reduced form: {}", equation));
	
	// Get solutions
	final_prints.extend(equation.solve());

	// Display result
	for line in final_prints.iter() {
		if !line.starts_with("[step]") || step_active {
			println!("{}", line);
		}
	}
}
