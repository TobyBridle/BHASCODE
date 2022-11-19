/*

THINGS THAT MUST BE TESTED:
- i.  The lexer must be able to lex a single token.
- ii. The lexer must be able to lex multiple tokens.
- iii. The lexer must be able to handle invalid tokens.
- iv. The lexer must be able to peek at the next token without consuming it.
- v. The lexer must be able to finish lexing the file and return an EOF token.
*/


#[cfg(test)]
mod lexer_tests {
    use rust_compiler::{Token, TokenType, Lexer, token, check_tokens};

    #[test]
    fn test_lexer_single_token() {
        let mut lexer_int = Lexer::new("123");
        check_tokens!(lexer_int, token!(NUMERIC, "123"));

        let mut lexer_float = Lexer::new("123.456");
        check_tokens!(lexer_float, token!(NUMERIC, "123.456"));

        let mut lexer_identifier = Lexer::new("abc");
        check_tokens!(lexer_identifier, token!(IDENTIFIER, "abc"));

        let mut lexer_operator = Lexer::new("+");
        check_tokens!(lexer_operator, token!(OPERATOR, "+"));
    }

    #[test]
    fn test_lexer_multiple_tokens() {
        let mut lexer = Lexer::new("int x = 10 + 20");
        check_tokens!(
            lexer,
            token!(IDENTIFIER, "int"),
            token!(IDENTIFIER, "x"),
            token!(OPERATOR, "="),
            token!(NUMERIC, "10"),
            token!(OPERATOR, "+"),
            token!(NUMERIC, "20")
        );
    }

    #[test]
    fn test_lexer_invalid_token() {
        let mut lexer = Lexer::new("int$$x = 10 + 20");
        check_tokens!(
            lexer,
            token!(IDENTIFIER, "int"),
            token!(UNKNOWN, "$$"),
            token!(IDENTIFIER, "x"),
            token!(OPERATOR, "="),
            token!(NUMERIC, "10"),
            token!(OPERATOR, "+"),
            token!(NUMERIC, "20")
        );
    }

    #[test]
    fn test_lexer_peek() {
        let mut lexer = Lexer::new("int x = 10 + 20");
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::IDENTIFIER);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::IDENTIFIER);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::IDENTIFIER);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::IDENTIFIER);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::IDENTIFIER);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::OPERATOR);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::OPERATOR);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::NUMERIC);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::NUMERIC);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::OPERATOR);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::OPERATOR);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::NUMERIC);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::NUMERIC);
        assert_eq!(lexer.peek().unwrap().unwrap().token, TokenType::EOF);
        assert_eq!(lexer.next().unwrap().unwrap().token, TokenType::EOF);
    }

    #[test]
    fn test_lexer_eof() {
        let mut lexer = Lexer::new("");
        check_tokens!(
            lexer,
            token!(EOF, "")
        );
    }

    #[test]
    fn test_lexer_eof_peek() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.peek(), None)
    }

    #[test]
    fn test_lexer_eof_next() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), Some(Ok(token!(EOF, ""))))
    }

    #[test]
    fn test_lexer_eof_with_whitespace() {
        let mut lexer = Lexer::new(" ");
        check_tokens!(
            lexer,
            token!(EOF, "")
        );
    }
}