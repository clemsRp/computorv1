
use clap::Parser;
use std::io;

mod config;
mod error;
mod maths;
mod fraction;
mod lexer;
mod polynomial;
mod parser;
mod equation;

use config::{FormatConfig, set_config};
use error::CalcResult;
use equation::Equation;

fn read_stdin_line() -> Option<String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(line.trim().to_string()),
    }
}

fn get_equation_solutions(input: &str) -> CalcResult<Vec<String>> {
	let mut final_prints: Vec<String> = Vec::new();

	// Create equation
	let mut equation = Equation::new(input)?;
	final_prints.push(format!("[step] Raw form: {}", equation));

	// Simplified it
	let sub = equation.clone();
	let sub_equation = format!("{} - ({}) = {} - ({})", sub.get_left(), sub.get_right(), sub.get_right(), sub.get_right());
	final_prints.push(format!("[step] Subtract from both sides: {}", sub_equation));
	equation = equation.reduced();
	final_prints.push(format!("Reduced form: {}", equation));
	
	// Get solutions
	let solved = equation.solve()?;
	final_prints.extend(solved);

	Ok(final_prints)
}

// Args parsing
#[derive(Parser)]
struct Cli {
    equation: Option<String>,

    #[arg(long)]
    step: bool,

	#[arg(short, long)]
    fraction: bool,

	#[arg(long)]
    simple: bool,
}

fn main() {
	// Parse arguments
	let cli = Cli::parse();
	set_config(FormatConfig { fraction: cli.fraction, simple: cli.simple });

	// Get input
	let input = match cli.equation {
        Some(equation) => equation,
        None => {
            let Some(line) = read_stdin_line() else {
                eprintln!("Error: no equation provided");
                std::process::exit(1);
            };
            line
        }
    };

	// Handle empty inputs
	if input.trim().is_empty() {
        eprintln!("Error: empty equation");
        std::process::exit(1);
    }

	// Get solutions
	let final_prints = get_equation_solutions(&input);
	match final_prints {
        Ok(prints) => {
			// Display result
			for line in prints.iter() {
				if !line.starts_with("[step]") || cli.step {
					println!("{}", line);
				}
			}
		},
        Err(error) => {
            error.report(&input);
			std::process::exit(1);
        }
    }
}
