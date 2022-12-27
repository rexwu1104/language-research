mod file;
mod token;
mod lexer;
mod tree;
mod parser;

fn main() {
    println!("{:#?}", parser::parse(lexer::lex("test.mx")));
}
