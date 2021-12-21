use crate::token_type::TokenType;
use crate::token::{Token, Literal};
use crate::lox::{LoxError};

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
            '!' => {
                let next_char = self.match_next('=');
                self.add_token(if next_char {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            },
            '=' => {
                let next_char = self.match_next('=');
                self.add_token(if next_char {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            },
            '<' => {
                let next_char = self.match_next('=');
                self.add_token(if next_char {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
            },
            '>' => {
                let next_char = self.match_next('=');
                self.add_token(if next_char {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
            },
            '/' => {
                let comment = self.match_next('/');
                if comment {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string()?,
            ch => {
                if self.is_digit(ch) {
                    self.number();
                } else if self.is_alpha(ch) {
                    self.identifier()
                } else {
                    return Err(LoxError{line: self.line, message: "Unexpected character.".to_string()})
                } 
            }
        }
        Ok(())
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) { self.advance(); }

        let text = &self.source[self.start as usize .. self.current as usize];
        let token_type = self.get_keyword(text);
        match token_type {
            Some(t) => self.add_token(t),
            _ => self.add_token(TokenType::Identifier)
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) { self.advance(); }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the ','
            self.advance();

            while self.is_digit(self.peek()) { self.advance(); }
        }

        let num: f64 = self.source[self.start as usize .. self.current as usize]
                .to_string()
                .parse()
                .unwrap();

        self.add_token_literal(TokenType::Number, Literal::Number(num))
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError{ line: self.line, message: "Unterminated string.".to_string() })
        }

        // The closing "
        self.advance();

        let value = self.source[(self.start+1) as usize .. (self.current-1) as usize].to_string();

        self.add_token_literal(TokenType::String, Literal::String(value));

        Ok(())

    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current as usize).unwrap() != expected { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {  
            return '\0';
        }

        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() as u32 {
            return '\0';
        }

        self.source.chars().nth((self.current + 1) as usize).unwrap()
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9' 
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
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

    fn get_keyword(&self, word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None
        }
    }
}