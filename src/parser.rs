
use crate::error::{CalcError, CalcResult};
use crate::config::BP_UNAIRE;
use crate::lexer::{TokenKind, Token};
use crate::polynomial::Polynomial;

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

fn exponent_of(poly: &Polynomial, pos: usize) -> CalcResult<i128> {
	let value = poly
        .as_constant()
        .map_err(|_| CalcError::at(pos, "Exponent must be a constant"))?;
    if !value.is_integer() {
        return Err(CalcError::at(pos, "Exponent must be an integer"));
    }

	Ok(value.get_num())
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser{tokens: tokens, index: 0}
	}

	pub fn parse_expr(&mut self, min_bind: u8) -> CalcResult<Polynomial> {
		self.parse_expr_depth(min_bind, 0)
	}

	fn parse_expr_depth(&mut self, min_bind: u8, depth: usize) -> CalcResult<Polynomial> {
		const MAX_DEPTH: usize = 256;
		if depth > MAX_DEPTH {
			return Err(CalcError::at(self.peek().get_pos(), "Expression is too deeply nested"));
		}

		// Get left part
		let token = self.next();
		let mut left_hand = match token.get_kind() {
			&TokenKind::Number(value) => Polynomial::constant(value),
			&TokenKind::X => Polynomial::x(),
			&TokenKind::Minus => {
				let pos = token.get_pos();
				(-self.parse_expr_depth(BP_UNAIRE, depth + 1)?).map_err(|e| e.at_if_none(pos))?
			},
			&TokenKind::LParen => {
				let lh = self.parse_expr_depth(0, depth + 1)?;
				self.expect(TokenKind::RParen)?;
				lh
			},
			&TokenKind::Plus => return Err(CalcError::at(token.get_pos(), "Unexpected operator, expected number, 'X', or '('")),
			TokenKind::End => return Err(CalcError::at(token.get_pos(), "Unexpected end of input, expected number, 'X', or '('")),
			_ => {
				return Err(CalcError::at(
					token.get_pos(),
					format!("Expected a number, 'X', or '(', found '{}'", token.get_text()),
				));
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
			let op_pos = op.get_pos();
			let right_hand_pos = self.peek().get_pos();
			let right_hand = self.parse_expr_depth(right_power, depth + 1)?;

			// Execute operator between left and right part
			let result = {
				match op.get_kind() {
					&TokenKind::Plus => (left_hand + right_hand).map_err(|e| e.at_if_none(op_pos))?,
					&TokenKind::Minus => (left_hand - right_hand).map_err(|e| e.at_if_none(op_pos))?,
					&TokenKind::Slash => (left_hand / right_hand).map_err(|e| e.at_if_none(op_pos))?,
					&TokenKind::Caret => {
						let exp = exponent_of(&right_hand, right_hand_pos)?;

						// Check exponent boundaries
						if exp < 0 {
                            return Err(CalcError::at(right_hand_pos, "Exponent cannot be negative"));
                        }
                        if exp > 1000 {
                            return Err(CalcError::at(right_hand_pos, "Exponent is too large"));
                        }

						// Convert it into a u32 to use it
                        let exp_u32: u32 = exp
                            .try_into()
                            .map_err(|_| CalcError::at(right_hand_pos, "Exponent is too large"))?;
						
						left_hand.pow(exp_u32).map_err(|e| e.at_if_none(op_pos))?
					},

					// Other case = explicite/implicite multiplication
					_ => (left_hand * right_hand).map_err(|e| e.at_if_none(op_pos))?,
				}
			};

			left_hand = result;
		}

		Ok(left_hand)
	}

	// Helpers
	fn peek(&self) -> &Token {
		&self.tokens[self.index]
	}

	fn next(&mut self) -> Token {
		let token = self.tokens[self.index].clone();
        if token.get_kind() != &TokenKind::End {
            self.index += 1;
        }
        token
	}

	pub fn expect(&mut self, exp_kind: TokenKind) -> CalcResult<()> {
		let next_token = self.next();
		if next_token.get_kind() != &exp_kind {
			let found = match next_token.get_kind() {
        	    TokenKind::End => "end of input".to_string(),
        	    _ => format!("'{}'", next_token.get_text()),
        	};
        	return Err(CalcError::at(
        	    next_token.get_pos(),
        	    format!("Expected {}, found {}", exp_kind.label(), found),
        	));
		}

		Ok(())
	}
}

