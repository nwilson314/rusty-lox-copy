use crate::token_type::TokenType;
use crate::token::{Token, Literal};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Scanner{
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token{
            token_type: TokenType::EOF, 
            lexeme: "".to_string(), 
            literal: Literal::None, 
            line: self.line
        });

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u32;
    }

}