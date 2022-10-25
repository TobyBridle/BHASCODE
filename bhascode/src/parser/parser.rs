use crate::Terminal;
pub use crate::{program_error, program_success, program_warn, Parser, ParserError, TokenType};

impl<'a> Parser<'_> {
    pub fn new(_src: &'a str) -> Parser<'a> {
        Parser {
            _src,
            tokens: vec![],
            expect: vec![],
            errors: vec![],
            lexer: crate::lexer::Lexer::new(_src),
        }
    }

    /// ### Parses source by lexing each token one at a time and converting it to a Terminal
    pub fn parse(&mut self) -> Option<()> {
        // Iterate over tokens and parse them as either identifiers or terminals
        let mut token = self.lexer.next_token();
        // Append to self.errors and continue if token is error
        Some(loop {
            if token.is_err() {
                let err = token.as_ref().err().unwrap();
                self.errors.push(ParserError::Generic {
                    raw: err.to_string(),
                    cur_line: self.lexer.cur_line,
                    cur_col: self.lexer.cur_col,
                });
            }

            match token {
                Ok(TokenType::EOF) => break,
                _ => {}
            }

            if *token.as_ref().unwrap() != TokenType::NOP {
                self.tokens.push(token.unwrap());
            }

            // match token {
            //     Ok(TokenType::Identifier(ident)) => {
            //         // Parse as identifier
            //         // -- Check if the token is a type / keyword
            //         match ident.as_str() {
            //             "int" => {
            //                 // Parse as int
            //                 program_success!("Parsed int", "Parser");
            //                 self.tokens.push(Terminal::Int);
            //
            //                 // Check that the next token is an identifier
            //                 self.expect
            //                     .push(Terminal::Identifier { raw: String::new() });
            //                 self.expect.push(Terminal::Char { c: '=' });
            //                 self.expect
            //                     .push(Terminal::IntLiteral { raw: String::new() });
            //             }
            //             // Must be a variable
            //             _ => {}
            //         }
            //     }
            //     Ok(TokenType::Punctuation { raw, kind }) => {
            //         // Parse as terminal
            //     }
            //     Ok(TokenType::Operator(c)) => {
            //         // Parse as operator
            //         if c == "-" {
            //             // Peek forward. If next token is another '-' then assume it is a comment
            //             // until the next newline
            //             let peek_token = self.lexer.peek_token();
            //             if peek_token.as_ref().unwrap_or(&TokenType::EOF)
            //                 == &TokenType::Operator("-".to_string())
            //             {
            //                 // Consume until the next newline
            //                 self.lexer.consume_until('\n');
            //             }
            //         }
            //     }
            //     Ok(TokenType::String(_)) => {
            //         // Parse as string
            //     }
            //     Ok(TokenType::Char(_)) => {
            //         // Parse as char
            //     }
            //     Ok(TokenType::Numeric { raw, hint }) => {
            //         // Parse as numeric
            //     }
            //     Ok(TokenType::Unknown(_)) => {
            //         // Parse as unknown
            //     }
            //     Ok(TokenType::Any) => {
            //         // Parse as any
            //     }
            //     Ok(TokenType::None) => {
            //         // Parse as none
            //     }
            //     Ok(TokenType::EOF) => {
            //         // Parse as EOF
            //     }
            //     _ => {
            //         // Parse as error
            //         let err = ParserError::Generic("Unknown token".to_string());
            //         program_error!(err, "Parser");
            //         return Err(err);
            //     }
            // }

            token = self.lexer.next_token();
        })
    }
}
