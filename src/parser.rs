
pub struct Parser {
	tokens: Vec<Token>,
	index: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser{tokens: tokens, index: 0}
	}

	fn peek(&self) -> Token {
		self.tokens[self.index]
	}

	fn seek(&self) -> Token {
		let token = self.tokens[self.index].clone();
        if token.kind != TokenKind::End {
            self.index += 1;
        }
        token
	}

	pub fn parse_expr() -> 
}
