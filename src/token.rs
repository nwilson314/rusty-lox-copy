use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal, // probably not suitable, but for now takes place of Java Object
    pub line: u32
}

impl Token {

}

#[derive(Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Identifier,
    None
}
