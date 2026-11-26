
use std::fmt;

use crate::error::CalcResult;
use crate::fraction::Fraction;
use crate::lexer::{TokenKind, tokenize};
use crate::polynomial::Polynomial;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Equation {
	left: Polynomial,
	right: Polynomial,
}

impl Equation {
    pub fn new(input: String) -> CalcResult<Self> {
        // Tokenize input
	    let tokens = tokenize(input)?;

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
		let left = self.left.clone() - self.right.clone();
		let right = self.right.clone() - self.right.clone();

		Equation {
            left: left,
	    	right: right,
	    }
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
			2 => self.handle_degree_two(&mut inter_print),
			_ => inter_print.push(String::from("The polynomial degree is strictly greater than 2, I can't solve.")),
		}

		inter_print
	}

	// Utils
	fn handle_degree_one(&self, inter_print: &mut Vec<String>) {
		inter_print.push(String::from("The solution is:"));
		let a = self.left.get_coeffs()[1];
		let b = self.left.get_coeffs()[0];

		// Get result
		let mut result = Fraction::default();
		result = result - b;
		result = result / a;

		inter_print.push(format!("{}", result));
	}

	fn handle_degree_two(&self, inter_print: &mut Vec<String>) {
		// Get variables
		let coeffs = self.left.get_coeffs();
		let a = coeffs[2];
		let b = coeffs[1];
		let c = coeffs[0];

		// Calculate Discriminant
		let left = b.pow(2);
		let right = Fraction::new(4, 1) * a * c;
		let discriminant = left - right;

		// Get solutions common part
		let mut common = Fraction::default();
		common = common - b;
		common = common / (Fraction::new(2, 1) * a);

		match discriminant {
			// Delta < 0 -> Imaginary
		    d if d.is_neg() => {
				let mut right = -discriminant.sqrt();
				right = right / (Fraction::new(2, 1) * a);

				inter_print.push(String::from("Discriminant is strictly negative, the two complex solutions are:"));
				inter_print.push(format!("{} + {}i/{}", common, right.get_num().abs(), right.get_denum()));
				inter_print.push(format!("{} - {}i/{}", common, right.get_num().abs(), right.get_denum()));
			},

			// Delta > 0 -> 2 distincts root
		    d if d.is_pos() => {
				let mut right = discriminant.sqrt();
				right = right / (Fraction::new(2, 1) * a);

				inter_print.push(String::from("Discriminant is strictly positive, the two solutions are:"));
				inter_print.push(format!("{}", common + right));
				inter_print.push(format!("{}", common - right));
			},

			// Delta = 0 -> 1 unique root
		    _ => {
				inter_print.push(String::from("Discriminant is null, the solution is:"));
				inter_print.push(format!("{}", common));
			},
		}
	}

	// Helpers
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
}

// Define Equation display
impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}
