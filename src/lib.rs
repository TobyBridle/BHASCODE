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

*/

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

- Shorthand for creating a token using a type and a &str value
    token!(token_type, token_value)

*/

#[macro_export]
macro_rules! check_tokens {
    ($lexer:expr, $($expected:expr),+) => {
        $(
            let token = $lexer.next().unwrap().unwrap();
            assert_eq!(token, $expected);
        )+
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