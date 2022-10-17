pub mod lexer;
pub use lexer::*;
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

    #[error("Cannot create Character Literal '{raw}'. {hint}")]
    CharLiteralError { raw: String, hint: Hints },

    #[error("Invalid escape sequence '\\{c}'")]
    InvalidEscapeSequence { c: char },
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
    String(String),
    Char(char),
    Numeric { raw: String, hint: Hints },
    Unknown(char), // Could also be read as unimplemented!
    Any,
    None,
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
    EmptyCharLiteral,
}

impl std::fmt::Display for Hints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hints::IntegerValue => write!(f, "Integer Value"),
            Hints::FloatingPointValue => write!(f, "Floating Point Value"),
            Hints::String => write!(f, "String"),
            Hints::NoHint => write!(f, "No Hint"),
            Hints::ExtraneousSymbol => write!(f, "Extraneous Symbol"),
            Hints::MissingExpectedSymbol => write!(f, "Missing Expected Symbol"),
            Hints::EmptyCharLiteral => write!(f, "Character Literal is Empty"),
        }
    }
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
