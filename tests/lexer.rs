// Import lexer_get_tokens macro
#[cfg(test)]
mod lexer_tests {
    use bhascode::*;
    #[test]
    pub fn test_decl() {
        let mut lexer: Lexer;
        let mut tokens: Vec<Token> = Vec::new();
        let mut correct_tokens: Vec<Token>;

        // Test Integer is lexed correctly
        lexer_get_tokens!("int a = 5", &mut tokens, lexer);

        correct_tokens = vec![
            TokenType::Identifier("int".to_string()),
            TokenType::Identifier("a".to_string()),
            TokenType::Operator("=".to_string()),
            TokenType::Numeric {
                raw: "5".to_string(),
                hint: Hints::IntegerValue,
            },
        ];

        assert_eq!(tokens, correct_tokens);
        tokens.clear();

        // Test Float is parsed
        lexer_get_tokens!("float b = 5.5", &mut tokens, lexer);
        correct_tokens = vec![
            TokenType::Identifier("float".to_string()),
            TokenType::Identifier("b".to_string()),
            TokenType::Operator("=".to_string()),
            TokenType::Numeric {
                raw: "5.5".to_string(),
                hint: Hints::FloatingPointValue,
            },
        ];

        assert_eq!(tokens, correct_tokens);
        tokens.clear();

        // Test String is parsed
        lexer_get_tokens!("string s = \"Hello World\"", &mut tokens, lexer);

        correct_tokens = vec![
            TokenType::Identifier("string".to_string()),
            TokenType::Identifier("s".to_string()),
            TokenType::Operator("=".to_string()),
            TokenType::String("Hello World".to_string()),
        ];
        assert_eq!(tokens, correct_tokens);
        tokens.clear();

        // Test Char is parsed
        lexer_get_tokens!("char c = 'a'", &mut tokens, lexer);
        correct_tokens = vec![
            TokenType::Identifier("char".to_string()),
            TokenType::Identifier("c".to_string()),
            TokenType::Operator("=".to_string()),
            TokenType::Char('a'),
        ];

        assert_eq!(tokens, correct_tokens);
        tokens.clear();
    }
    #[test]
    pub fn test_comments() {
        let mut lexer: Lexer;
        let mut tokens: Vec<Token> = Vec::new();
        lexer_get_tokens!(
            "-- this is a comment... please ignore me\n-- this is also a comment!!!''int",
            &mut tokens,
            lexer
        );

        // Remove all NOP tokens
        tokens.retain(|x| *x != TokenType::NOP);
        assert!(tokens.len() == 0);
        tokens.clear();

        lexer_get_tokens!("-- this is a comment... please ignore me\n-- this is also a comment!!!''\nint x = 5 -- this should be parsed still!!!", &mut tokens, lexer);
        let correct_tokens = vec![
            TokenType::Identifier("int".to_string()),
            TokenType::Identifier("x".to_string()),
            TokenType::Operator("=".to_string()),
            TokenType::Numeric {
                raw: "5".to_string(),
                hint: Hints::IntegerValue,
            },
        ];

        tokens.retain(|x| *x != TokenType::NOP);
        assert_eq!(tokens, correct_tokens);
        tokens.clear();
    }

    // Ensure that the state of the lexer does not get permanently altered by the functoin call
    #[test]
    pub fn test_lookahead_state() {
        let mut lexer = Lexer::new("int x = 5");
        let mut lexer_clone = Lexer::new("int x = 5");

        let peeked = lexer.peek_token().unwrap();
        let next_token = lexer.next_token().unwrap();

        assert_eq!(next_token, lexer_clone.next_token().unwrap());
        assert_eq!(peeked, next_token);
    }
}
