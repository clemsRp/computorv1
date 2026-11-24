
use crate::lexer::{TokenKind, Token};
use crate::fraction::Fraction;
use crate::polynomial::Polynomial;

pub struct Parser {
	tokens: Vec<Token>,
	index: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser{tokens: tokens, index: 0}
	}

	fn peek(&self) -> Token {
		self.tokens[self.index].clone()
	}

	fn next(&mut self) -> Token {
		let token = self.tokens[self.index].clone();
        if token.get_kind() != &TokenKind::End {
            self.index += 1;
        }
        token
	}

	pub fn parse_expr(&mut self) -> Result<Polynomial, String> {
		let poly = Polynomial::constant(Fraction::one());
		Ok(poly)
	}

	pub fn expect(&mut self, exp_kind: TokenKind) -> Result<(), String> {
		let next_token = self.next();
		if next_token.get_kind() != &exp_kind {
			return Err(format!("expected {:?}, found {:?}", exp_kind, next_token.get_kind()));
		}

		Ok(())
	}
}

