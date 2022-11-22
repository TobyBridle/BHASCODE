pub mod lexer;
pub use lexer::*;

// pub mod parser;
// pub use parser::*;

/*

####################################################################################

                    BASIC MACROS FOR IMPROVING CODE READABILITY

####################################################################################

i.  `#[macro_export]` is used to export the macro to the crate root.
ii. `#[macro_use]` is used to import the macro from the crate root.

------------------------------------------------------------------------------------

LIST OF IMPLEMENTED MACROS:

- Produce a prettier error message
    tagged_error(tag, message)

------------------------------------------------------------------------------------

*/

#[macro_export]
macro_rules! tagged_error {
    ($tag: expr, $msg: expr) => {
        format!(
            "\x1b[31;1mError: <{}> \x1b[31;3m\x1b[31m{}\x1b[0;0m",
            $tag, $msg
        )
    };
}

/*

####################################################################################

                    MACROS FOR IMPROVING TEST CASES READABILITY

####################################################################################

i.  `#[macro_export]` is used to export the macro to the crate root.
ii. `#[macro_use]` is used to import the macro from the crate root.

------------------------------------------------------------------------------------

LIST OF IMPLEMENTED MACROS:

- Check if the token(s) are as expected.
    check_tokens!(lexer, expected_tokens)

- Shorthand for consuming tokens using a closure
    consume!(lexer, token_vec, closure)
    EXAMPLE USAGE:
        // This will consume all tokens that are not NOP.
        consume!(lexer, token_vec, |token| token.token != TokenType::NOP)

- Shorthand for creating a token using a type and a &str value
    token!(token_type, token_value)
*/

#[macro_export]
macro_rules! check_tokens {
    ($lexer:expr, $($expected:expr),+) => {
        $(
            let token = $lexer.next().unwrap();
            // Check if the found token is NOP
            if token.token != TokenType::NOP {
                println!("{:?}", token);
                assert_eq!(token, $expected);
            }
        )+
    };
}

#[macro_export]
macro_rules! consume {
    ($lexer:expr, $tokens:expr, $closure:expr) => {
        for token in $lexer {
            if $closure(token.clone()) {
                $tokens.push(token);
            }
        }
    };
}

#[macro_export]
macro_rules! token {
    ($token_type:ident, $token_value:expr) => {
        Token {
            token: TokenType::$token_type,
            value: $token_value.to_string(),
        }
    };
}
