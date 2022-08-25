extern crate bhascode;

use bhascode::lexer::*;
fn main() {
    let mut lexer = Lexer::new("5 1.05 1e5 2ee (){]}");
    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(tok) => println!("{:?}", tok),
            Err(e) => println!("e is {:?}", e),
        }
    }
}
