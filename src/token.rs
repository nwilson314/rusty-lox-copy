use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal, // probably not suitable, but for now takes place of Java Object
    line: u32
}

impl Token {

}

pub enum Literal {
    String(String),
    Number(f64),
    Identifier,
    None
}
