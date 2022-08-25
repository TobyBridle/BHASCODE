use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Error attempting to read file: {0}")]
    FileIO(#[from] std::io::Error),

    #[error("Was expecting {expected:?}, found {found:?} instead.")]
    MissingExpectedSymbol { expected: TokenType, found: Token },

    #[error("Depth for Symbol {symbol:?} is already 0")]
    MisbalancedSymbol { symbol: char, open: char },

    #[error("Found unknown symbol {symbol:?}")]
    UnknownSymbol { symbol: String },

    #[error("Cannot create Numeric Literal due to invalid character {raw:?}")]
    NumericLiteralError { raw: String, hint: Hints },
}

pub type Token = TokenType;
#[derive(Debug)]
pub enum TokenType {
    // End of Token Stream
    EOF,

    // ., [, (
    Punctuation { raw: char, kind: PunctuationKind },
    // -, +, *
    Operator(String),
    Identifier(String),
    Char(char),
    Numeric { raw: String, hint: Hints },
    Unknown(char), // Could also be read as unimplemented!
}

#[derive(Debug)]
pub enum PunctuationKind {
    // (
    Open(usize),

    // )
    Close(usize),

    // , ;
    Seperator,
}

#[derive(Debug)]
pub enum Hints {
    IntegerValue,
    FloatingPointValue,
    String,

    NoHint,
    ExtraneousSymbol,
    MissingExpectedSymbol,
}

pub struct Lexer<'a> {
    // Tracking in a Human-Readable Format
    pub cur_line: usize,
    pub cur_col: usize,

    // Codepoint Offset (Bytes Read)
    pub cp_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>,
    punctuation_state: std::collections::HashMap<char, usize>,
}

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

    /// Iterates over a string and attmepts to parse a number
    ///
    /// * `start`:
    fn parse_number(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut seen_decimal_point = false;
        let mut seen_expression = false;
        let mut num = start.to_string();
        let radix = 10;

        if start == '.' {
            return Err(LexerError::NumericLiteralError {
                raw: num.clone(),
                hint: Hints::FloatingPointValue,
            });
        }

        loop {
            match self.chars.peek() {
                Some(c) if *c == '.' && seen_decimal_point => {
                    num.push(*c);
                    self.consume_char();
                    return Err(LexerError::NumericLiteralError {
                        raw: num.clone(),
                        hint: Hints::ExtraneousSymbol,
                    });
                }
                Some(c) if *c == '.' && !seen_decimal_point && !seen_expression => {
                    seen_decimal_point = true;
                    num.push(*c);
                    self.consume_char();
                }
                Some(c) if *c == 'e' || *c == 'E' && !seen_expression => {
                    seen_expression = true;
                    num.push(*c);
                    self.consume_char();

                    match self.chars.peek() {
                        Some(c) if *c == '+' || *c == '-' => {
                            num.push(*c);
                            self.consume_char();
                        }
                        _ => {}
                    }

                    match self.chars.peek() {
                        Some(c) if c.is_whitespace() => {
                            return Err(LexerError::NumericLiteralError {
                                raw: num.clone(),
                                hint: Hints::MissingExpectedSymbol,
                            });
                        }
                        Some(c) if !c.is_digit(radix) => {
                            num.push(*c);
                            self.consume_char();
                            return Err(LexerError::NumericLiteralError {
                                raw: num,
                                hint: Hints::ExtraneousSymbol,
                            });
                        }
                        None => {
                            return Err(LexerError::NumericLiteralError {
                                raw: num,
                                hint: Hints::MissingExpectedSymbol,
                            });
                        }
                        _ => {}
                    }
                }
                Some(c) if c.is_digit(radix) => {
                    num.push(*c);
                    self.consume_char();
                }
                Some(c) if c.is_ascii_alphabetic() || c.is_digit(10) => {
                    num.push(*c);
                    return Err(LexerError::NumericLiteralError {
                        raw: num,
                        hint: Hints::String,
                    });
                }
                _ => {
                    break Ok(TokenType::Numeric {
                        raw: num,
                        hint: if seen_decimal_point || seen_expression {
                            Hints::FloatingPointValue
                        } else {
                            Hints::IntegerValue
                        },
                    });
                }
            };
        }
    }

    /// Transform a generic character to the correct token
    ///
    /// * `c`:
    fn transform_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' | '{' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Open(self.push_open_punctuation(c)),
            }),
            ')' | ']' | '}' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Close(self.push_close_punctuation(c)?),
            }),
            '0'..='9' => Ok(self.parse_number(c)?),
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
