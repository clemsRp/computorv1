
use std::fmt;

use crate::config::get_config;
use crate::error::CalcResult;
use crate::fraction::Fraction;
use crate::lexer::{TokenKind, tokenize};
use crate::polynomial::Polynomial;
use crate::parser::Parser;
use crate::maths::{abs, acos, cbrt, cos, from_f64, sqrt, to_f64, PI};

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
			3 => self.handle_degree_three(&mut inter_print)?,
			_ => inter_print.push(String::from("The polynomial degree is strictly greater than 3, I can't solve.")),
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

	fn handle_degree_three(&self, inter_print: &mut Vec<String>) -> CalcResult<()> {
	    // Get variables
	    let coeffs = self.left.get_coeffs();
	    let a = coeffs[3];
	    let b = coeffs[2];
	    let c = coeffs[1];
	    let d = coeffs[0];
	    inter_print.push(String::from("[step] Equation is 'ax³ + bx² + cx + d' form:"));
	    inter_print.push(format!("[step] a = {}, b = {}, c = {}, d = {}", a, b, c, d));

	    // Calculate Discriminant
	    let t1 = ((((Fraction::new(18, 1) * a)? * b)? * c)? * d)?;
	    let t2 = ((Fraction::new(4, 1) * b.pow(3)?)? * d)?;
	    let t3 = (b.pow(2)? * c.pow(2)?)?;
	    let t4 = ((Fraction::new(4, 1) * a)? * c.pow(3)?)?;
	    let t5 = ((Fraction::new(27, 1) * a.pow(2)?)? * d.pow(2)?)?;
	    let discriminant = ((((t1 - t2)? + t3)? - t4)? - t5)?;
	    inter_print.push(format!("[step] Discriminant (18abcd - 4b³d + b²c² - 4ac³ - 27a²d²): {}", discriminant));

	    // Get depressed cubic variables (t³ + pt + q = 0, with x = t - b/3a)
	    let (af, bf, cf, df) = (to_f64(a), to_f64(b), to_f64(c), to_f64(d));
	    let shift = bf / (3.0 * af);
	    let p = (3.0 * af * cf - bf * bf) / (3.0 * af * af);
	    let q = (2.0 * bf * bf * bf - 9.0 * af * bf * cf + 27.0 * af * af * df) / (27.0 * af * af * af);
	    inter_print.push(format!("[step] Depressed form t³ + pt + q (x = t - b/3a): p = {}, q = {}", from_f64(p)?, from_f64(q)?));

	    match discriminant {
	        // Delta > 0 -> 3 distincts real roots
	        disc if disc.is_pos() => {
	            let radius = 2.0 * sqrt(-p / 3.0);
	            let angle = acos((3.0 * q / (2.0 * p)) * sqrt(-3.0 / p)) / 3.0;

	            inter_print.push(String::from("Discriminant is strictly positive, the three solutions are:"));

	            // Add the three real roots
	            for k in 0..3 {
	                let t = radius * cos(angle - 2.0 * (k as f64) * PI / 3.0);
	                inter_print.push(format!("{}", from_f64(t - shift)?));
	            }
	        },

	        // Delta < 0 -> 1 real root and 2 complex roots
	        disc if disc.is_neg() => {
	            let root = sqrt(q * q / 4.0 + p * p * p / 27.0);
	            let u = cbrt(-q / 2.0 + root);
	            let v = cbrt(-q / 2.0 - root);

	            let real_root = from_f64(u + v - shift)?;
	            let common = from_f64(-(u + v) / 2.0 - shift)?;
	            let imag = from_f64(abs(sqrt(3.0) / 2.0 * (u - v)))?.format_imaginary();
	            let config = get_config();

	            inter_print.push(String::from("Discriminant is strictly negative, the real solution and the two complex solutions are:"));
	            inter_print.push(format!("{}", real_root));

	            // Add imaginary solutions
	            if common.is_zero() && config.simple {
	                inter_print.push(format!("{}", imag));
	                inter_print.push(format!("-{}", imag));
	            } else {
	                inter_print.push(format!("{} + {}", common, imag));
	                inter_print.push(format!("{} - {}", common, imag));
	            }
	        },

	        // Delta = 0 -> multiple root (exact computation with fractions)
	        _ => {
	            let delta_zero = (b.pow(2)? - ((Fraction::new(3, 1) * a)? * c)?)?;

	            if delta_zero.is_zero() {
	                // Triple root
	                let triple = ((-b)? / (Fraction::new(3, 1) * a)?)?;

	                inter_print.push(String::from("Discriminant is null, the triple solution is:"));
	                inter_print.push(format!("{}", triple));
	            } else {
	                // Double root + simple root
	                let double_num = ((Fraction::new(9, 1) * a)? * d)? - (b * c)?;
	                let double_root = (double_num? / (Fraction::new(2, 1) * delta_zero)?)?;

	                let simple_a = (((Fraction::new(4, 1) * a)? * b)? * c)?;
	                let simple_b = ((Fraction::new(9, 1) * a.pow(2)?)? * d)?;
	                let simple_num = ((simple_a - simple_b)? - b.pow(3)?)?;
	                let simple_root = (simple_num / (a * delta_zero)?)?;

	                inter_print.push(String::from("Discriminant is null, the double solution and the simple solution are:"));
	                inter_print.push(format!("{}", double_root));
	                inter_print.push(format!("{}", simple_root));
	            }
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
