mod file;
mod token;
mod lexer;

fn main() {
    println!("{:?}", lexer::lex("test.mx"));
}
