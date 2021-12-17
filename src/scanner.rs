use crate::token_type::TokenType;
use crate::token::{Token, Literal};
use crate::lox::LoxError;

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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError>  {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token{
            token_type: TokenType::EOF, 
            lexeme: "".to_string(), 
            literal: Literal::None, 
            line: self.line
        });

        Ok(self.tokens.to_vec())
    }

    fn scan_token(&mut self) -> Result<(), LoxError>{
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            _ch => {
                return Err(LoxError{line: self.line, message: "Unexpected character.".to_string()})
            }
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u32;
    }

    fn advance(&mut self) -> char {
        let temp_char =  self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        temp_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Literal::None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start as usize .. self.current as usize];

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line
        })
    }

}