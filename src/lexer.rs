
use crate::error::{CalcError, CalcResult};
use crate::fraction::Fraction;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(Fraction),
    X,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Equal,
    End,
}

impl TokenKind {
	pub fn label(&self) -> &'static str {
		match self {
			TokenKind::Equal => "'='",
			TokenKind::Plus => "'+'",
			TokenKind::Minus => "'-'",
			TokenKind::Star => "'*'",
			TokenKind::Slash => "'/'",
			TokenKind::Caret => "'^'",
			TokenKind::LParen => "'('",
			TokenKind::RParen => "')'",
			TokenKind::End => "end of input",
			TokenKind::X => "'X'",
			TokenKind::Number(_) => "a number",
		}
	}
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    text: String,
	pos: usize,
}

impl Token {
	pub fn get_kind(&self) -> &TokenKind {
		&self.kind
	}

	pub fn get_text(&self) -> &str {
		&self.text
	}

	pub fn get_pos(&self) -> usize {
		self.pos
	}
}

pub fn tokenize(input: &str) -> CalcResult<Vec<Token>> {
    let raw_tokens: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;
    while i < raw_tokens.len() {
        // Get token datas
        let c = raw_tokens[i];
        let (kind, len) = match c {
			// Skip spaces
			' ' | '\t' | '\n' | '\r' => {
			    i += 1;
			    continue;
			}

			// Numbers
			'0'..='9' => {
				let (value, len) = read_number(&raw_tokens, i)?;
				(TokenKind::Number(value), len)
			}
			
			// Variables
			'X' | 'x' => (TokenKind::X, 1),

			// Operators
			'+' => (TokenKind::Plus, 1),
			'-' => (TokenKind::Minus, 1),
			'*' => (TokenKind::Star, 1),
			'/' => (TokenKind::Slash, 1),
			'^' => (TokenKind::Caret, 1),
			'=' => (TokenKind::Equal, 1),
			'(' => (TokenKind::LParen, 1),
			')' => (TokenKind::RParen, 1),

			// Invalid char
			_ => return Err(CalcError::at(i, format!("Invalid character '{c}'"))),
        };

        // Add new token
        let text = raw_tokens[i..i + len].iter().collect();
        tokens.push(Token {kind: kind, text: text, pos: i});

        i += len;
    }

	// Terminate tokens
	tokens.push(Token{kind: TokenKind::End, text: String::new(), pos: raw_tokens.len()});

    Ok(tokens)
}

fn read_number(raw_tokens: &[char], start: usize) -> CalcResult<(Fraction, usize)> {
	let mut base_digits = String::new();
	let mut frac_digits = String::new();

	let t_len = raw_tokens.len();
	let mut i = start;

	// Get base digits
	while i < t_len && raw_tokens[i].is_ascii_digit() {
		base_digits.push(raw_tokens[i]);
		i += 1;
	}

	if i < t_len && raw_tokens[i] == '.' {
		// Get frac digits
		i += 1;
		while i < t_len && raw_tokens[i].is_ascii_digit() {
			frac_digits.push(raw_tokens[i]);
			i += 1;
		}

		// Check non empty list
		if frac_digits.is_empty() {
			return Err(CalcError::at(i, "Missing numbers after '.'"));
		}
	}

	if i < t_len && raw_tokens[i] == '.' {
		return Err(CalcError::at(i, "Too many '.' characters"));
	}

	let value = Fraction::from_decimale(base_digits, frac_digits).
		map_err(|e| e.at_if_none(start))?;

	Ok((value, i - start))
}
