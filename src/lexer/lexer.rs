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
                // If the character is a digit, then we need to continue reading the number until we reach a non-digit character.
                // We can then return the number as a token.
                '0'..='9' => Some(self.lex_numeric(character)),
                _ => return Some(Err(format!("Unknown token: {}", character))),
            }
        } else {
            return Some(Ok(token!(EOF, "")));
        }
    }

    fn take(self, n: usize) -> std::iter::Take<lexer::Lexer<'a>> {
        let input_str = self.input.as_str();
        Lexer {
            input: input_str.chars(),
            codepoint: self.codepoint,
            
            line: self.line,
            column: self.column,
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
        }
    }

    pub fn peek(&mut self) -> Option<TokenResult> {
        if self.input.as_str().is_empty() {
            return None
        }
        
        let mut lexer = Lexer {
            input: self.input.as_str().chars(),
            codepoint: self.codepoint,
            line: self.line,
            column: self.column,
        };
        lexer.next()
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
}