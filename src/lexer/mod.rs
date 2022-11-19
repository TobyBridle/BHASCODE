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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token: TokenType, value: String) -> Self {
        Self { token, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // We do not handle keywords at this stage, so we can just use a generic identifier token for all keywords.
    IDENTIFIER,

    // All operators are stored under the generic operator token.
    OPERATOR,

    // All numeric values are stored under the generic numeric token.
    // This means that at this stage, we are not sure whether the value is an integer or a float.
    NUMERIC,

    // The types of braces are not important for the parser, so we can just use a generic brace token.
    BRACE,

    // All unknown tokens are stored under the generic unknown token.
    UNKNOWN,

    // NOP is a special token that is used when the lexer has scanned
    // something that is not a useful token, but also not an error.
    NOP,
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

    // We can use a stack to store the braces that we have opened.
    // This will allow us to check if the braces are balanced.
    // If we find a closing brace, we can pop the last brace from the stack.
    // If the stack is empty, then we know that we have an unbalanced brace.
    // If the stack is not empty, then we can check if the brace that we have found matches the last brace in the stack.
    // If the braces do not match, then we know that we have an unbalanced brace.
    // If the braces do match, then we can pop the last brace from the stack.
    braces_balancer: Vec<char>,
    
}