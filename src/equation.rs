
use crate::lexer::{TokenKind, tokenize};
use crate::polynomial::Polynomial;
use crate::parser::Parser;

pub struct Equation {
	left: Polynomial,
	right: Polynomial,
}

impl Equation {
	pub fn print(&self) {
		self.left.print();
		print!(" = ");
		self.right.print();
	}
}

impl Equation {
    pub fn new(input: String) -> Result<Self, String> {
        // Tokenize input
	    let tokens = tokenize(input);

	    // Init parser
	    let mut parser = Parser::new(tokens);
        
	    // Get equation parts
	    let left = parser.parse_expr()?;
	    parser.expect(TokenKind::Equal)?;
	    let right = parser.parse_expr()?;
	    parser.expect(TokenKind::End)?;
        
	    Ok(Equation {
            left: left,
	    	right: right,
	    })
    }
}
