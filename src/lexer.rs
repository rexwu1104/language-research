use crate::token::Token;

pub fn lex(file_name: &str) -> Vec<Token> {
    super::token::tokenize(super::file::read_raw(file_name))
}