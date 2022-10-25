use crate::{lexer::*, program_error};

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
    /// * `c`: char
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
    /// * `c`: char
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
    /// * `c`: char
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

    /// # Continues until a whitespace character or invalid var/keyword char is found (e.g $)
    /// ## Places the characters into a buffer which is then placed inside a String token
    /// * `c`: char
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
                    self.consume_char();
                }
                _ => break Ok(TokenType::Identifier(buf)),
            }
        }
    }

    /// ## Places all characters into a String token buffer until a non-escaped double-quote is
    /// found
    fn parse_string(&mut self) -> Result<TokenType, LexerError> {
        let mut buf = String::new();
        let mut has_found_escape = false;

        loop {
            match self.consume_char() {
                Some('"') if !has_found_escape => return Ok(TokenType::String(buf)),
                Some(c) if c == '\\' => {
                    has_found_escape = true;
                }
                Some(c) if c == '\n' => {
                    return Err(LexerError::UnexpectedTermination {
                        found: TokenType::String(format!("{}\n", buf)),
                    });
                }
                Some(c) => {
                    buf.push(c);
                    has_found_escape = false;
                }
                None => {
                    return Err(LexerError::MissingExpectedSymbol {
                        expected: TokenType::Char('"'),
                        found: TokenType::String(buf),
                    })
                }
            }
        }
    }

    /// # Places all characters into a String token buffer until a non-escaped single-quote is found. #
    /// ## If the ending buffer is longer than 1 char (with the exception of escape sequences) an
    /// Err is returned
    fn parse_char(&mut self) -> Result<TokenType, LexerError> {
        // Buf until we find a single quote
        let mut buf = String::new();
        let mut has_found_escape = false;

        loop {
            let c = self.chars.next();
            match c {
                Some('\'') if !has_found_escape => break,
                Some(c) if c == '\\' => {
                    has_found_escape = true;
                }
                Some(c) => {
                    buf.push(c);
                    has_found_escape = false;
                }
                None => {
                    let err = LexerError::MissingExpectedSymbol {
                        expected: TokenType::Char('\''),
                        found: TokenType::None,
                    };
                    return Err(err);
                }
            }
        }

        let c = buf.chars().next();
        if buf.len() > 1 && !has_found_escape {
            let err = LexerError::CharLiteralError {
                raw: buf,
                hint: Hints::ExtraneousSymbol,
            };
            return Err(err);
        } else {
            // Convert buf to char (e.g "\n" -> '\n')
            if c.is_none() {
                let err = LexerError::CharLiteralError {
                    raw: buf,
                    hint: Hints::EmptyCharLiteral,
                };
                // self.cur_col - 1 because we want to point to the start of the char literal
                program_error!(
                    format!("{}:{} - {}", self.cur_line, self.cur_col - 1, err),
                    "Lexer"
                );
                return Err(err);
            }
            return Ok(TokenType::Char(c.unwrap()));
        }
    }

    /// Iterates over a string and attmepts to parse a number
    ///
    /// * `start`: char
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
    /// * `c`: char
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
            '+' | '*' | '/' | '%' | '=' => Ok(TokenType::Operator(c.to_string())),

            '-' => {
                // If the next char is also a '-' it is a comment. Skip until next line begins
                if let Some(next) = self.chars.peek() {
                    if *next == '-' {
                        self.consume_until('\n');
                        Ok(TokenType::NOP)
                    } else {
                        Ok(TokenType::Operator(c.to_string()))
                    }
                } else {
                    Ok(TokenType::Operator(c.to_string()))
                }
            }

            // Strings
            '"' => Ok(self.parse_string()?),

            // Char
            '\'' => Ok(self.parse_char()?),

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

    /// Skips all whitespace ascii characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume_char();
        }
    }

    /// ## Consumes a character to begin the transformation of characters into tokens.
    /// ### Stops if next token is EOF
    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();
        if let Some(c) = self.consume_char() {
            return self.transform_type(c);
        } else {
            Ok(TokenType::EOF)
        }
    }

    /// ### Returns (peeks) the next token without permanently changing state of the lexer
    /// ## Gets the next token, and then resets all of the offsets and lines to their previous states
    pub fn peek_token(&mut self) -> Result<TokenType, LexerError> {
        // Grab next token without permanently altering state of the lexer
        let _cur_col = self.cur_col;
        let _cur_line = self.cur_line;
        let _cp_offset = self.cp_offset;
        let _punctuation_state = self.punctuation_state.clone();
        let _chars = self.chars.clone();

        let token = self.next_token();

        self.cur_col = _cur_col;
        self.cur_line = _cur_line;
        self.cp_offset = _cp_offset;
        self.punctuation_state = _punctuation_state;
        self.chars = _chars;

        Ok(token?)
    }

    /// ### Consumes all characters until the given character is found
    ///
    /// * `pattern`: char
    pub fn consume_until(&mut self, pattern: char) {
        while let Some(c) = self.chars.peek() {
            if *c == pattern {
                self.consume_char();
                break;
            }
            self.consume_char();
        }
    }
}
