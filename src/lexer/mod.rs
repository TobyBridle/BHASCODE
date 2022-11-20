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
    // We will handle keywords seperately to identifiers.
    // They will both go through the lex_identifier() function
    // but we will check a map of keywords to see if the identifier
    // is a keyword or not.
    KEYWORD,
    IDENTIFIER,

    // Strings are enclosed in double quotes.
    STRING,

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

#[macro_export]
macro_rules! token_from_keyword {
    ($keyword:expr) => {
        if KEYWORDS.contains_key($keyword.as_str()) {
            token!(KEYWORD, $keyword.to_string())
        } else {
            token!(IDENTIFIER, $keyword.to_string())
        }
    };
}

/*

##########################################################################################

                            CONSTANTS FOR THE COMPILER.

##########################################################################################

*/

// The list of keywords that the compiler will recognise.
// This is a static, constant map that will be used to check if an identifier is a keyword or not.
// The map is created using the lazy_static crate.
// This means that the map will only be created once, when the program is first run.
// This is more efficient than creating the map every time the compiler is run.
// We can access the map in other modules by using the KEYWORDS constant.

lazy_static::lazy_static! {
    pub static ref KEYWORDS: std::collections::HashMap<&'static str, TokenType> = {
        let mut map = std::collections::HashMap::new();
            map.insert("int", TokenType::KEYWORD);
            // map.insert("float", TokenType::KEYWORD);
            // map.insert("string", TokenType::KEYWORD);
            // map.insert("bool", TokenType::KEYWORD);
            map
    };
}

// The list of operators that the compiler will recognise.
// This is a static, constant map that will be used to check if an operator is a valid operator or not.
// The map is created using the lazy_static crate.
// This means that the map will only be created once, when the program is first run.
// This is more efficient than creating the map every time the compiler is run.
// We can access the map in other modules by using the OPERATORS constant.

lazy_static::lazy_static! {
    // The key is String, not &str, because we need to be able to use the `contains_key()` method.
    // The `contains_key()` method takes a reference to a string, so we need to use a String.

    pub static ref OPERATORS: std::collections::HashMap<String, TokenType> = {
        let mut map = std::collections::HashMap::new();
            map.insert("+".to_string(), TokenType::OPERATOR);
            map.insert("-".to_string(), TokenType::OPERATOR);
            map.insert("*".to_string(), TokenType::OPERATOR);
            map.insert("/".to_string(), TokenType::OPERATOR);
            map.insert("%".to_string(), TokenType::OPERATOR);
            map.insert("!".to_string(), TokenType::OPERATOR);
            map.insert("=".to_string(), TokenType::OPERATOR);

            map.insert("++".to_string(), TokenType::OPERATOR);
            map.insert("--".to_string(), TokenType::OPERATOR);
            map.insert("+=".to_string(), TokenType::OPERATOR);
            map.insert("-=".to_string(), TokenType::OPERATOR);
            map.insert("*=".to_string(), TokenType::OPERATOR);
            map.insert("/=".to_string(), TokenType::OPERATOR);

            map.insert("==".to_string(), TokenType::OPERATOR);
            map.insert("!=".to_string(), TokenType::OPERATOR);
            map.insert("<".to_string(), TokenType::OPERATOR);
            map.insert(">".to_string(), TokenType::OPERATOR);
            map.insert("<=".to_string(), TokenType::OPERATOR);
            map.insert(">=".to_string(), TokenType::OPERATOR);
            map.insert("&&".to_string(), TokenType::OPERATOR);
            map.insert("||".to_string(), TokenType::OPERATOR);

            map
    };
}
