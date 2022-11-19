use std::char;

use crate::lexer::*;

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(character) = self.input.next() {
            // We need to skip the whitespace & comments
            if character.is_whitespace() {
                self.codepoint += 1;
                self.input.next();
                return self.next();
            }

            match character {

                // If the character is an open brace, we can give it a unique id.
                // We can then check if there is a corresponding close brace.
                // If not, then we can throw an error before we even start parsing.
                '{' | '(' | '[' => Some(self.lex_open_brace(character)),

                // If the character is a close brace, we can check if it matches the corresponding open brace.
                // If not, then we can throw an error before we even start parsing.
                '}' | ')' | ']' => Some(self.lex_close_brace(character)),

                // If the character is an operator, we need to check if it is a single or a double character operator.
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' => Some(self.lex_operator(character)),

                // If the character is a digit, then we need to continue reading the number until we reach a non-digit character.
                // We can then return the number as a token.
                '0'..='9' => Some(self.lex_numeric(character)),

                // If the character is a letter, then we need to continue reading the identifier until we reach a non-letter character.
                // We can then return the identifier as a token.
                'a'..='z' | 'A'..='Z' | '_' => Some(self.lex_identifier(character)),

                // If the character is a `#`, we know it is the start of either a single or multi-line comment
                '#' => Some(self.lex_comment(character)),

                _ => return Some(Err(format!("Unknown token: {}", character))),
            }
        } else {
            return None;
        }
    }

    fn take(self, n: usize) -> std::iter::Take<lexer::Lexer<'a>> {
        let input_str = self.input.as_str();
        Lexer {
            input: input_str.chars(),
            codepoint: self.codepoint,
            
            line: self.line,
            column: self.column,

            braces_balancer: self.braces_balancer,
        }.take(n)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
            codepoint: 0,
            
            line: 1,
            column: 1,

            braces_balancer: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Option<TokenResult> {
        if self.input.as_str().is_empty() {
            return None;
        }
        
        let mut lexer = Lexer {
            input: self.input.as_str().chars(),
            codepoint: self.codepoint,
            
            line: self.line,
            column: self.column,

            braces_balancer: self.braces_balancer.clone(),
        };
        lexer.next()
    }

    fn lex_open_brace(&mut self, character: char) -> TokenResult {
        self.codepoint += 1;
        self.braces_balancer.push(character);
        Ok(token!(BRACE, character))
    }

    fn lex_close_brace(&mut self, character: char) -> TokenResult {
        self.codepoint += 1;
        // We want to check if the close brace matches the corresponding open brace.
        // We need to peek at the stack (braces_balancer) to see if the last open brace is the same as the current close brace.
        // If it is, then we can pop the last open brace from the stack.
        // If it is not, then we can throw an error.
        let opposite_character = match character {
            '}' => '{',
            ')' => '(',
            ']' => '[',
            _ => return Err(format!("Unknown token: {}", character)),
        };

        if let Some(brace_token) = self.braces_balancer.iter().peekable().peek() {
            if brace_token == &&opposite_character {
                self.braces_balancer.pop();
                return Ok(token!(BRACE, character));
            } else {
                return Err(format!("Mismatched braces: {} and {}", brace_token, character));
            }
        } else {
            return Err(format!("Unmatched close brace: {}", character));
        }
        
    }

    fn lex_operator(&mut self, character: char) -> TokenResult {
        let mut operator = character.to_string();

        // We need to check if the next character is also an operator.
        // If it is, then we need to add it to the operator string.
        if let Some(next_character) = self.input.next() {
            if next_character.is_whitespace() {
                self.codepoint += 1;
                return Ok(token!(OPERATOR, operator));
            }

            match next_character {
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' => {
                    operator.push(next_character);
                }
                _ => {
                    return Ok(token!(OPERATOR, operator))
                }
            }
        }

        Ok(token!(OPERATOR, operator))
    }

    fn lex_numeric(&mut self, character: char) -> TokenResult {
        let mut value = String::new();
        let mut has_point = false;

        value.push(character);

        while let Some(character) = self.input.next() {
            if character.is_numeric() || ( character == '.' && !has_point ) {
                value.push(character);
            } else {
                break;
            }
        }

        Ok(token!(NUMERIC, value))
    }

    fn lex_identifier(&mut self, character: char) -> TokenResult {
        let mut buf = String::new();
        buf.push(character);

        // We want to continue until we get to a character that cannot be used
        // in an identifier.
        // This includes whitespace, operators, braces and punctuation (with the exclusion of `_`).
        while let Some(character) = self.input.next() {
            match character {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => buf.push(character),
                c if c.is_whitespace() => {
                    return Ok(token!(IDENTIFIER, buf));
                },
                // It is not a valid identifier character nor is it whitespace.
                // We need to return an error and stop lexing.
                _ => return Err(format!("Character `{}` cannot be used in an identifier", character)),
            }
        }

        self.input.next();

        Ok(token!(IDENTIFIER, buf))
    }

    
    fn lex_comment(&mut self, character: char) -> TokenResult {
        // TODO: Add support for other platforms

        let mut is_multi_line = false;
        if let Some(next_character) = self.input.next() {
            if next_character == '-' {
                is_multi_line = true;
            }
        }

        // If multi-line, continue until we find a `-#` sequence.
        // If single-line, continue until we find a newline.
        if is_multi_line {
            while let Some(character) = self.input.next() {
                if character == '-' {
                    if let Some(next_character) = self.input.next() {
                        if next_character == '#' {
                            return Ok(token!(NOP, ""));
                        }
                    }
                }
            }
            return Err("Unterminated multi-line comment".to_string());
        } else {
            while let Some(character) = self.input.next() {
                if character == '\n' {
                    return Ok(token!(NOP, ""));
                }
            }
        return Ok(token!(NOP, ""));
        }
    }
}