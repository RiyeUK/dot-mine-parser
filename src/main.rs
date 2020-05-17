mod lexer;
mod token;

use lexer::Lexer;
use token::Token;

use std::fs;

fn main() {

    let filename = "Test.rbt";
    let code     = fs::read_to_string(filename);
    let mut lexer    = Lexer::new(code.unwrap());
    
    loop {
        match &lexer.next_token() {
            Ok(token) => {
                println!("{:?}",token);
                if token == &Token::EOF { break; }
            },
            Err(e) => {
                println!("[LEXER ERROR]: {}", e);
                panic!();
            }
        }
    }

}
