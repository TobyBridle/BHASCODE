use crate::lexer::*;
use crate::tagged_error;
use crate::token_from_keyword;

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    /// Retrieves the next tokens from the lexer (or None if there are no more tokens)
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(character) = self.skip_to_next_char() {
            let token: Option<TokenResult> = match character {
                // If the character is an open brace, we can give it a unique id.
                // We can then check if there is a corresponding close brace.
                // If not, then we can throw an error before we even start parsing.
                '{' | '(' | '[' => Some(self.lex_open_brace(character)),

                // If the character is a close brace, we can check if it matches the corresponding open brace.
                // If not, then we can throw an error before we even start parsing.
                '}' | ')' | ']' => Some(self.lex_close_brace(character)),

                // We can then return the number as a token.
                '0'..='9' => Some(self.lex_numeric(character)),

                // If the character is a double quote, then it must be the start of a string.
                // We can then continue reading the string until we reach another double quote.
                '"' => Some(self.lex_string()),

                // If the character is a letter, then we need to continue reading the identifier until we reach a non-letter character.
                // We can then return the identifier as a token.
                'a'..='z' | 'A'..='Z' | '_' => Some(self.lex_identifier(character)),

                // If the character is a `#`, we know it is the start of either a single or multi-line comment
                '#' => Some(self.lex_comment()),

                _ => {
                    // It is either an operator or an error.
                    Some(self.lex_operator(character))
                }
            };

            /* TODO: NICER ERROR MESSAGES
             * E.g lexing ".123" returns an error with something such as:
             *      "Error: <LEXER> Unexpected token `.` at 0:0"
             * We need to find the errors and convert them into an Error token with the value
             * of tagged_error!("LEXER", error.unwrap())
             * Somehow, we need to format the error message by including the line and column number at
             * which the error occured.
             * When we return the Err(val) from a function, we need to be careful with the ownership.
             * All functions take lexer as a mutable argument and thus take ownership too.
             * */
            /* INFO: Handling errors automatically
             * We want to pass the token to a closure which can return either a valid Token or an
             * Error Token
             * */
            let handle_err = |token: Option<TokenResult>| -> Token {
                if let Some(token) = token {
                    return token.unwrap_or_else(|err| token!(ERR, tagged_error!("LEXER", err)));
                } else {
                    token!(ERR, tagged_error!("LEXER", "Unexpected error occured!"))
                }
            };
            return Some(handle_err(token));
        } else {
            return None;
        }
    }
}

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given input string
    ///
    /// * `input`:
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),

            line: 1,
            column: 1,

            braces_balancer: Vec::new(),
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        if let Some(character) = self.input.next() {
            if character == '\n' {
                self.column = 0;
                self.line += 1;
            } else {
                self.column += 1;
            }
            return Some(character);
        }
        None
    }

    /// Skips to the next character in the input, ignoring whitespace.
    fn skip_to_next_char(&mut self) -> Option<char> {
        while let Some(character) = self.input.by_ref().peekable().peek() {
            if !character.is_whitespace() {
                return Some(*character);
            } else {
                return self.next_char();
            }
        }
        None
    }

    /// Returns the next token in the lexer without consuming it
    pub fn peek(&mut self) -> Option<Token> {
        if self.input.as_str().is_empty() {
            return None;
        }

        let mut lexer = Lexer {
            input: self.input.as_str().chars(),

            line: self.line,
            column: self.column,

            braces_balancer: self.braces_balancer.clone(),
        };
        lexer.next()
    }

    /// Peeks at the next character in the input without consuming it.
    fn peek_character(&mut self) -> String {
        let c = self.input.to_owned().peekable().peek().copied();
        if c.is_some() {
            return c.unwrap().to_string();
        } else {
            return String::new();
        }
    }

    /// Places an opening brace on the stack and returns the corresponding token
    ///
    /// * `character`:
    fn lex_open_brace(&mut self, character: char) -> TokenResult {
        self.braces_balancer.push(character);
        Ok(token!(BRACE, character))
    }

    /// Attempts to pop a matching opening brace from the stack of braces to balance the braces.
    ///
    /// * `character`:
    fn lex_close_brace(&mut self, character: char) -> TokenResult {
        // We want to check if the close brace matches the corresponding open brace.
        // We need to peek at the stack (braces_balancer) to see if the last open brace is the same as the current close brace.
        // If it is, then we can pop the last open brace from the stack.
        // If it is not, then we can throw an error.
        let opposite_character = match character {
            '}' => '{',
            ')' => '(',
            ']' => '[',
            _ => {
                return Err(format!(
                    "Unknown token: {} at {}",
                    character,
                    self.get_position()
                ))
            }
        };

        if let Some(brace_token) = self.braces_balancer.iter().peekable().peek() {
            if brace_token == &&opposite_character {
                self.braces_balancer.pop();
                return Ok(token!(BRACE, character));
            } else {
                return Err(format!(
                    "Mismatched braces: {} and {} at {}",
                    brace_token,
                    character,
                    self.get_position()
                ));
            }
        } else {
            return Err(format!(
                "Unmatched close brace: {} at {}",
                character,
                self.get_position()
            ));
        }
    }

    /// Lexes an operator token by comparing the current char and the next with the keys of the
    /// OPERATORS map.
    ///
    /// * `character`:
    fn lex_operator(&mut self, character: char) -> TokenResult {
        // Check if the current character is an operator.
        // If not, then attempt with the current character & the next character as a string
        // If both of these fail, we know that there is no chance of the characters being operator
        // tokens
        let operators = character.to_string() + &self.peek_character();
        if OPERATORS.contains_key(&character.to_string()) && !OPERATORS.contains_key(&operators) {
            Ok(token!(OPERATOR, character))
        } else {
            if OPERATORS.contains_key(&operators) {
                self.next_char();
                Ok(token!(OPERATOR, operators))
            } else {
                Err(format!(
                    "Unknown token: {} at {}",
                    character,
                    self.get_position()
                ))
            }
        }
    }

    /// Lexes an operator token by consuming the input until non-digit characters (with the
    /// exception of the decimal point) are found.
    ///
    /// * `character`:
    fn lex_numeric(&mut self, character: char) -> TokenResult {
        let mut value = String::new();
        let mut has_point = false;

        value.push(character);

        while let Some(character) = self.next_char() {
            if character.is_numeric() || (character == '.' && !has_point) {
                value.push(character);
            } else {
                // We can check the next character too to check if there is a syntax error.
                // For example, `1.2.3` is not a valid number, and `123abc` is not a valid identifier
                // (because it starts with a number).
                if OPERATORS.contains_key(&character.to_string()) || character.is_whitespace() {
                    return Ok(token!(NUMERIC, value));
                } else {
                    return Err(format!("Invalid number: {}", value));
                }
            }
        }

        Ok(token!(NUMERIC, value))
    }

    /// Lexes a string token by consuming the input until an unescaped closing quote is found.
    ///
    /// * `character`:
    fn lex_string(&mut self) -> TokenResult {
        let mut str = String::new();
        let mut escaped = false;

        // Check if the next character is a double quote and that the escape flag is false.
        // If it is, then we can return an empty string as a token.
        while let Some(next_character) = self.input.to_owned().peekable().peek() {
            if next_character == &'"' && !escaped {
                self.next_char();
                return Ok(token!(STRING, str));
            } else if next_character == &'\\' {
                escaped = true;
                self.next_char();
            } else {
                escaped = false;
                str.push(self.next_char().unwrap());
            }
        }

        Ok(token!(STRING, str))
    }

    /// Lexes an identifier token by consuming the input until a non-alphanumeric character is found (excluding `_`)
    ///
    /// * `character`:
    fn lex_identifier(&mut self, character: char) -> TokenResult {
        let mut buf = String::new();
        buf.push(character);

        // We want to continue until we get to a character that cannot be used
        // in an identifier.
        // This includes whitespace, operators, braces and punctuation (with the exclusion of `_`).
        while let Some(character) = self.next_char() {
            match character {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => buf.push(character),
                c if c.is_whitespace() => {
                    // We can return the identifier as a token.
                    // However, we to do a check to see if the identifier is a keyword.
                    // If it is, we can return the keyword as a token.
                    // If it is not, we can return the identifier as a token.
                    // Keywords are stored inside the `KEYWORDS` map which is a constant static map.
                    // We can use the .contains_key() method to check if the identifier is a keyword.
                    // The macro token_from_keyword!() is used to determine whether or not the identifier is a keyword or not.
                    // It returns a token with the correct type (either KEYWORD or IDENTIFIER).
                    return Ok(token_from_keyword!(&buf));
                }
                // It is not a valid identifier character nor is it whitespace.
                // We need to return an error and stop lexing.
                _ => {
                    return Err(format!(
                        "Character `{}` cannot be used in an identifier",
                        character
                    ))
                }
            }
        }

        self.next_char();

        Ok(token_from_keyword!(&buf))
    }

    /// Skips comments by consuming until a newline is found (single-line comments) or a closing
    /// `-#` is found (multi-line comments).
    ///
    /// * `character`:
    fn lex_comment(&mut self) -> TokenResult {
        let mut is_multi_line = false;
        if let Some(next_character) = self.next_char() {
            if next_character == '-' {
                is_multi_line = true;
            }
        }

        // If multi-line, continue until we find a `-#` sequence.
        // If single-line, continue until we find a newline.
        if is_multi_line {
            while let Some(character) = self.next_char() {
                if character == '-' {
                    if let Some(next_character) = self.next_char() {
                        if next_character == '#' {
                            return Ok(token!(NOP, ""));
                        }
                    }
                }
            }
            return Err("Unterminated multi-line comment".to_string());
        } else {
            while let Some(character) = self.next_char() {
                if character == '\n' {
                    break;
                }
            }
            return Ok(token!(NOP, ""));
        }
    }

    fn get_position(&self) -> String {
        format!("{}:{}", self.line, self.column)
    }
}
