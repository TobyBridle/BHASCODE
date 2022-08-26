extern crate bhascode;

use bhascode::lexer::*;
fn main() {
    let mut lexer = Lexer::new("print(\"hello\")");
    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(tok) => println!("{:?}", tok),
            Err(e) => println!("{:?}", e),
        }
    }
}
