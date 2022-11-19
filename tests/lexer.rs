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
    use rust_compiler::{Token, TokenType, Lexer, token, check_tokens, consume, TokenResult};

    #[test]
    fn test_lexer_single_token() {
        let mut lexer_int = Lexer::new("123");
        check_tokens!(lexer_int, token!(NUMERIC, "123"));

        let mut lexer_float = Lexer::new("123.456");
        check_tokens!(lexer_float, token!(NUMERIC, "123.456"));
        
        let mut lexer_operator = Lexer::new("+");
        check_tokens!(lexer_operator, token!(OPERATOR, "+"));

        let mut lexer_brace = Lexer::new("(){}[]");
        check_tokens!(
            lexer_brace,
            token!(BRACE, "("),
            token!(BRACE, ")"),
            token!(BRACE, "{"),
            token!(BRACE, "}"),
            token!(BRACE, "["),
            token!(BRACE, "]")
        );

        let mut lexer_identifier = Lexer::new("abc");
        check_tokens!(lexer_identifier, token!(IDENTIFIER, "abc"));

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
        let mut lexer_invalid_ident = Lexer::new("int$$x = 10 + 20");
        // It should throw an error for the invalid token.
        assert!(lexer_invalid_ident.next().unwrap().is_err());

        let mut lexer_invalid_brace = Lexer::new("{}[])]}");
        
        // It should throw an error for the invalid token.
        // Skip the first 4 tokens because they are valid.
        for _ in 0..4 {
            lexer_invalid_brace.next().unwrap();
        }

        assert!(lexer_invalid_brace.next().unwrap().is_err());
        
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
        assert!(lexer.peek().is_none());
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_lexer_eof() {
        let mut lexer = Lexer::new("");
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_lexer_eof_peek() {
        let mut lexer = Lexer::new("");
        assert!(lexer.peek().is_none())
    }

    #[test]
    fn test_lexer_eof_next() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), None)
    }

    #[test]
    fn test_lexer_eof_with_whitespace() {
        let mut lexer = Lexer::new(" ");
        assert!(lexer.next().is_none());

    }

    #[test]
    fn test_lexer_comments() {
        let mut lexer_single_line = Lexer::new("# This is a single-line comment\nint x = 5");
        // Create vector that can be passed to consume! macro
        let mut tokens: Vec<Token> = Vec::new();
        consume!(lexer_single_line, tokens, |tok: Token| tok.token != TokenType::NOP);
        assert_eq!(tokens.len(), 4);

        let mut lexer_multi_line = Lexer::new("#- This is a multi-line comment\n int x = 5 -#");
        tokens.clear();
        consume!(lexer_multi_line, tokens, |tok: Token| tok.token != TokenType::NOP);
        assert_eq!(tokens.len(), 0);

        let mut lexer_multi_line_with_tokens = Lexer::new("#- This is a multi-line comment\n int x = 5 -# int y = 10");
        tokens.clear();
        consume!(lexer_multi_line_with_tokens, tokens, |tok: Token| tok.token != TokenType::NOP);
        assert_eq!(tokens.len(), 4);
    }
}