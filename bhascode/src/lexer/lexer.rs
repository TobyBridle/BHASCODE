use crate::lexer::*;

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            cur_line: 1,
            cur_col: 1,
            cp_offset: 0,
            chars: chars.chars().peekable(),
            punctuation_state: std::collections::HashMap::new(),
        }
    }

    /// Maps a piece of punctuation to its matching symbol
    ///
    /// * `c`:
    fn map_punctuation(c: char) -> char {
        match c {
            '}' => '{',
            '{' => '}',
            ']' => '[',
            '[' => ']',
            ')' => '(',
            '(' => ')',
            _ => panic!("Unknown Punctuation: {}", c),
        }
    }

    /// Increments the state of a given punctuation symbol
    ///
    /// * `c`:
    fn push_open_punctuation(&mut self, c: char) -> usize {
        if let Some(i) = self.punctuation_state.get_mut(&c) {
            *i += 1;
            return *i - 1;
        }
        self.punctuation_state.insert(c, 1);
        return 0;
    }

    /// Decrements the state of a given punctuation symbol
    ///
    /// * `c`:
    fn push_close_punctuation(&mut self, c: char) -> Result<usize, LexerError> {
        if let Some(i) = self.punctuation_state.get_mut(&Lexer::map_punctuation(c)) {
            if *i >= 1 {
                *i -= 1;
                Ok(*i)
            } else {
                Err(LexerError::MisbalancedSymbol {
                    symbol: (c),
                    open: Lexer::map_punctuation(c),
                })
            }
        } else {
            Err(LexerError::MisbalancedSymbol {
                symbol: (c),
                open: Lexer::map_punctuation(c),
            })
        }
    }

    fn parse_identifier(&mut self, c: char) -> Result<TokenType, LexerError> {
        let mut buf = String::new();
        buf.push(c);

        loop {
            match self.chars.peek() {
                Some(c) if c.is_ascii_punctuation() && *c != '_' => {
                    break Ok(TokenType::Identifier(buf))
                }

                Some(c) if c.is_ascii_alphanumeric() || *c == '_' => {
                    buf.push(*c);
                    self.chars.next();
                }
                _ => break Ok(TokenType::Identifier(buf)),
            }
        }
    }
    fn parse_string(&mut self) -> Result<TokenType, LexerError> {
        let mut buf = String::new();
        let mut has_found_escape = false;

        loop {
            match self.chars.next() {
                Some('"') if !has_found_escape => return Ok(TokenType::String(buf)),
                Some(c) if c == '\\' => {
                    has_found_escape = true;
                }
                Some(c) => {
                    buf.push(c);
                    has_found_escape = false;
                }
                None => {
                    return Err(LexerError::MissingExpectedSymbol {
                        expected: TokenType::Char('"'),
                        found: TokenType::None,
                    })
                }
            }
        }
    }

    /// Iterates over a string and attmepts to parse a number
    ///
    /// * `start`:
    fn parse_number(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut seen_decimal_point = false;
        let mut seen_expression = false;

        let mut raw = start.to_string();
        let mut hint = Hints::IntegerValue;
        let radix = 10;

        if start == '.' {
            hint = Hints::MissingExpectedSymbol;
            return Err(LexerError::NumericLiteralError { raw, hint });
        }

        loop {
            match self.chars.peek() {
                Some(c) if *c == '.' && seen_decimal_point => {
                    raw.push(*c);
                    self.consume_char();
                    hint = Hints::ExtraneousSymbol;
                    return Err(LexerError::NumericLiteralError { raw, hint });
                }
                Some(c) if *c == '.' && !seen_decimal_point && !seen_expression => {
                    seen_decimal_point = true;
                    hint = Hints::FloatingPointValue;
                    raw.push(*c);
                    self.consume_char();
                }
                Some(c) if *c == 'e' || *c == 'E' && !seen_expression => {
                    seen_expression = true;
                    hint = Hints::FloatingPointValue;
                    raw.push(*c);
                    self.consume_char();

                    match self.chars.peek() {
                        Some(c) if *c == '+' || *c == '-' => {
                            raw.push(*c);
                            self.consume_char();
                        }
                        _ => {}
                    }

                    match self.chars.peek() {
                        Some(c) if c.is_whitespace() => {
                            hint = Hints::MissingExpectedSymbol;
                            return Err(LexerError::NumericLiteralError { raw, hint });
                        }
                        Some(c) if !c.is_digit(radix) => {
                            raw.push(*c);
                            self.consume_char();
                            hint = Hints::ExtraneousSymbol;
                            return Err(LexerError::NumericLiteralError { raw, hint });
                        }
                        None => {
                            hint = Hints::MissingExpectedSymbol;
                            return Err(LexerError::NumericLiteralError { raw, hint });
                        }
                        _ => {}
                    }
                }
                Some(c) if c.is_digit(radix) => {
                    raw.push(*c);
                    self.consume_char();
                }
                Some(c) if c.is_ascii_alphabetic() || c.is_digit(10) => {
                    raw.push(*c);
                    hint = Hints::String;
                    return Err(LexerError::NumericLiteralError { raw, hint });
                }
                _ => {
                    break Ok(TokenType::Numeric { raw, hint });
                }
            };
        }
    }

    /// Transform a generic character to the correct token
    ///
    /// * `c`:
    fn transform_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            // Punctuation
            '(' | '[' | '{' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Open(self.push_open_punctuation(c)),
            }),
            ')' | ']' | '}' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Close(self.push_close_punctuation(c)?),
            }),

            // Identifiers / Keywords
            c if c.is_ascii_alphabetic() || c == '_' => Ok(self.parse_identifier(c)?),

            // Parsing Numbers
            '0'..='9' => Ok(self.parse_number(c)?),

            // Math Operators
            '+' | '-' | '*' | '/' | '%' | '=' => Ok(TokenType::Operator(c.to_string())),

            // Strings
            '"' => Ok(self.parse_string()?),

            _ => Err(LexerError::UnknownSymbol {
                symbol: (c.to_string()),
            }),
        }
    }

    /// Consumes a character and updates the current position
    fn consume_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.cp_offset += 1;
                self.cur_col += 1;
                if c == '\n' {
                    self.cur_line += 1;
                    self.cur_col = 1;
                }
                return Some(c);
            }
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume_char();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();
        if let Some(c) = self.consume_char() {
            return self.transform_type(c);
        } else {
            Ok(TokenType::EOF)
        }
    }
}
