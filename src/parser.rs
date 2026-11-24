
use crate::lexer::{TokenKind, Token};
use crate::polynomial::Polynomial;

const BP_UNAIRE: u8 = 5;

pub struct Parser {
	tokens: Vec<Token>,
	index: usize,
}

fn get_binding(token: &Token) -> Option<(u8, u8)> {
	match token.get_kind() {
		TokenKind::Plus | TokenKind::Minus => Some((1, 2)),
		TokenKind::Slash | TokenKind::Star | TokenKind::X | TokenKind::LParen => Some((3, 4)),
		TokenKind::Caret => Some((8, 7)),
		_ => None,
	}
}

fn exponent_of(poly: &Polynomial) -> Result<i128, String> {
	let value = poly.as_constant()?;
	if !value.is_integer() {
		return Err(String::new());
	}

	Ok(value.get_num())
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

	pub fn parse_expr(&mut self, min_bind: u8) -> Result<Polynomial, String> {
		// Get left part
		let token = self.next();
		let mut left_hand = match token.get_kind() {
			&TokenKind::Number(value) => Polynomial::constant(value),
			&TokenKind::X => Polynomial::x(),
			&TokenKind::Minus => self.parse_expr(BP_UNAIRE)?.negative(),
			&TokenKind::Plus => self.parse_expr(BP_UNAIRE)?,
			&TokenKind::LParen => {
				let lh = self.parse_expr(0)?;
				self.expect(TokenKind::RParen)?;
				lh
			},
			_ => {
				return Err(String::new());
			},
		};

		// Get right part going recursively if needed
		loop {
			// Get operator
			let op = self.peek().clone();
			let Some((left_power, right_power)) = get_binding(&op) else {
				break;
			};
			if left_power < min_bind {
				break;
			}

			// Consume operator if needed -> non implicite
			let implicite = matches!(op.get_kind(), &TokenKind::X | &TokenKind::LParen);
			if !implicite {
				self.next();
			}

			// Parse right part
			let right_hand = self.parse_expr(right_power)?;

			// Execute operator between left and right part
			let result = {
				match op.get_kind() {
					&TokenKind::Plus => left_hand.add(&right_hand),
					&TokenKind::Minus => left_hand.sub(&right_hand),
					&TokenKind::Slash => left_hand.div(&right_hand),
					&TokenKind::Caret => {
						let exp = exponent_of(&right_hand)?;
						left_hand.pow(exp.try_into().unwrap())
					},

					// Other case = explicite/implicite multiplication
					_ => left_hand.mul(&right_hand),
				}
			};

			left_hand = result;
		}

		Ok(left_hand)
	}

	pub fn expect(&mut self, exp_kind: TokenKind) -> Result<(), String> {
		let next_token = self.next();
		if next_token.get_kind() != &exp_kind {
			return Err(format!("expected {:?}, found {:?}", exp_kind, next_token.get_kind()));
		}

		Ok(())
	}
}

