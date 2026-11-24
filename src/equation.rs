
use std::fmt;

use crate::lexer::{TokenKind, tokenize};
use crate::polynomial::Polynomial;
use crate::parser::Parser;

pub struct Equation {
	pub left: Polynomial,
	pub right: Polynomial,
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
}
