use crate::token_type::TokenType;

pub struct Token {
    toke_type: TokenType,
    lexeme: String,
    literal: Literal // probably not suitable, but for now takes place of Java Object
}

impl Token {

}

enum Literal {
    String(String),
    Number(f64),
    Identifier,
    None
}
