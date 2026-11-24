
use std::fmt;

use crate::fraction::Fraction;
use crate::lexer::{TokenKind, tokenize};
use crate::polynomial::Polynomial;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Equation {
	left: Polynomial,
	right: Polynomial,
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}

impl Equation {
    pub fn new(input: String) -> Result<Self, String> {
        // Tokenize input
	    let tokens = tokenize(input);

	    // Init parser
	    let mut parser = Parser::new(tokens);
        
	    // Get equation parts
	    let left = parser.parse_expr(0)?;
	    parser.expect(TokenKind::Equal)?;
	    let right = parser.parse_expr(0)?;
	    parser.expect(TokenKind::End)?;
        
	    Ok(Equation {
            left: left,
	    	right: right,
	    })
    }

	pub fn reduced(&self) -> Self {
		let left = self.left.sub(&self.right);
		let right = self.right.sub(&self.right);

		Equation {
            left: left,
	    	right: right,
	    }
	}

	pub fn get_degree(&self) -> usize {
		let l_coeffs = self.left.get_coeffs();
		let r_coeffs = self.right.get_coeffs();

		let degree = l_coeffs.len().max(r_coeffs.len()) - 1;

		degree.max(0)
	}

	pub fn get_left(&self) -> Polynomial {
		self.left.clone()
	}

	pub fn get_right(&self) -> Polynomial {
		self.right.clone()
	}

	pub fn solve(&self) -> Vec<String> {
		let mut inter_print: Vec<String> = Vec::new();
		let degree = self.get_degree();

		// Handle special degree
		if degree == 0 {
			let end = {
				if !self.get_left().is_zero() {
					"No solution."
				} else {
					"Any real number is a solution."
				}
			};
			inter_print.push(String::from(end));
			return inter_print;
		}

		inter_print.push(format!("Polynomial degree: {}", degree));

		match degree {
			1 => self.handle_degree_one(&mut inter_print),
			// 2 => self.handle_degree_two(&inter_print),
			_ => panic!("vntgmlrdp"),
		}

		inter_print
	}

	fn handle_degree_one(&self, inter_print: &mut Vec<String>) {
		inter_print.push(String::from("The solution is:"));
		let a = self.left.get_coeffs()[1];
		let b = self.left.get_coeffs()[0];

		// Get result
		let mut result = Fraction::default();
		result = result.sub(&b);
		result = result.div(&a);

		inter_print.push(format!("{}", result));
	}
}
