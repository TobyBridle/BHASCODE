pub mod lexer;
pub use lexer::*;

/*

######################################################

  _____    ___    _  __  _____   _   _   ____  
 |_   _|  / _ \  | |/ / | ____| | \ | | / ___| 
   | |   | | | | | ' /  |  _|   |  \| | \___ \ 
   | |   | |_| | | . \  | |___  | |\  |  ___) |
   |_|    \___/  |_|\_\ |_____| |_| \_| |____/ 

`Token` is a structure with accessible properties `.token` and `.value`. The `.token` property is an enum of all possible tokens, and the `.value` property is a string slice of the token's value.

When we call `lexer.next()` we get a `Result` with a `Token` inside. We can then use the `unwrap()` method to get the `Token` out of the `Result`. We can then use the `assert_eq!` macro to compare the `.token` property of the `Token` with the expected `Token` enum.
If the lexer could not find a token, it will return an `Err` with a `String` inside. We can use the `unwrap_err()` method to get the `String` out of the `Err`.

######################################################

*/

use crate::token;

pub type TokenResult = Result<Token, String>;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token: TokenType, value: String) -> Self {
        Self { token, value }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // We do not handle keywords at this stage, so we can just use a generic identifier token for all keywords.
    IDENTIFIER,

    // All operators are stored under the generic operator token.
    OPERATOR,

    // All numeric values are stored under the generic numeric token.
    // This means that at this stage, we are not sure whether the value is an integer or a float.
    NUMERIC,

    // All unknown tokens are stored under the generic unknown token.
    UNKNOWN,

    // The end of file token is used to indicate that the lexer has reached the end of the file.
    EOF,
}

/*


  _       _____  __  __  _____   ____  
 | |     | ____| \ \/ / | ____| |  _ \ 
 | |     |  _|    \  /  |  _|   | |_) |
 | |___  | |___   /  \  | |___  |  _ < 
 |_____| |_____| /_/\_\ |_____| |_| \_\
                                       
The lexer will be implemented as an iterator. This means that we can use the `for` loop to iterate over the tokens in the file.

*/

pub struct Lexer<'a> {

    input: std::str::Chars<'a>,
    codepoint: usize,

    line: usize,
    column: usize,
}