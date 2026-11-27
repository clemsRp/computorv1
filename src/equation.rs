
use std::fmt;

use crate::config::get_config;
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
    pub fn new(input: &str) -> CalcResult<Self> {
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

	pub fn reduced(&self) -> CalcResult<Self> {
		let left = (self.left.clone() - self.right.clone())?;
		let right = Polynomial::default();

		Ok(Equation {
            left: left,
	    	right: right,
	    })
	}

	pub fn solve(&self) -> CalcResult<Vec<String>> {
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
			return Ok(inter_print);
		}

		inter_print.push(format!("Polynomial degree: {}", degree));

		match degree {
			1 => self.handle_degree_one(&mut inter_print)?,
			2 => self.handle_degree_two(&mut inter_print)?,
			_ => inter_print.push(String::from("The polynomial degree is strictly greater than 2, I can't solve.")),
		}

		Ok(inter_print)
	}

	// Utils
	fn handle_degree_one(&self, inter_print: &mut Vec<String>) -> CalcResult<()>{
		let a = self.left.get_coeffs()[1];
		let b = self.left.get_coeffs()[0];

		inter_print.push(String::from("[step] Equation is 'ax + b' form:"));
		inter_print.push(format!("[step] a = {}, b = {}", a, b));
		inter_print.push(String::from("[step] Root = -b / a"));
		inter_print.push(String::from("The solution is:"));

		// Get result
		let result = ((-b)? / a)?;

		inter_print.push(format!("{}", result));

		Ok(())
	}

	fn handle_degree_two(&self, inter_print: &mut Vec<String>) -> CalcResult<()> {
		// Get variables
		let coeffs = self.left.get_coeffs();
		let a = coeffs[2];
		let b = coeffs[1];
		let c = coeffs[0];
		inter_print.push(String::from("[step] Equation is 'ax² + bx + c' form:"));
		inter_print.push(format!("[step] a = {}, b = {}, c = {}", a, b, c));

		// Calculate Discriminant
		let left = b.pow(2)?;
		let right = ((Fraction::new(4, 1) * a)? * c)?;
		let discriminant = (left - right)?;
		inter_print.push(format!("[step] Discriminant (b² - 4ac): {}", discriminant));

		// Get solutions common part
		let two_a = (Fraction::new(2, 1) * a)?;
		let common = ((-b)? / two_a)?;

		match discriminant {
			// Delta < 0 -> Imaginary
		    d if d.is_neg() => {
				let right = ((-discriminant)?.sqrt()? / two_a)?;
				let imag = right.format_imaginary();
				let config = get_config();

				inter_print.push(String::from("Discriminant is strictly negative, the two complex solutions are:"));

				// Add imaginary solutions
				if common.is_zero() && config.simple {
					inter_print.push(format!("{}", imag));
					inter_print.push(format!("-{}", imag));
				} else {
					inter_print.push(format!("{} + {}", common, imag));
					inter_print.push(format!("{} - {}", common, imag));
				}
			},

			// Delta > 0 -> 2 distincts root
		    d if d.is_pos() => {
				let right = (discriminant.sqrt()? / two_a)?;

				inter_print.push(String::from("Discriminant is strictly positive, the two solutions are:"));
				inter_print.push(format!("{}", (common + right)?));
				inter_print.push(format!("{}", (common - right)?));
			},

			// Delta = 0 -> 1 unique root
		    _ => {
				inter_print.push(String::from("Discriminant is null, the solution is:"));
				inter_print.push(format!("{}", common));
			},
		}

		Ok(())
	}

	// Helpers
	pub fn get_degree(&self) -> usize {
		let l_coeffs = self.left.get_coeffs();
		let r_coeffs = self.right.get_coeffs();

		l_coeffs.len().max(r_coeffs.len()).saturating_sub(1)
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
