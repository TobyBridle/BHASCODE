pub mod parser;
pub use crate::lexer::TokenType;
use crate::{program_error, program_warn};
pub use parser::*;

use thiserror::Error;

#[derive(Debug)]
pub enum ParserError {
    Generic {
        raw: String,
        cur_line: usize,
        cur_col: usize,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::Generic {
                raw,
                cur_line,
                cur_col,
            } => write!(f, "{1}:{2} - {0}", raw, cur_line, cur_col),
        }
    }
}

pub enum Terminal {
    Int,
    Identifier { raw: String },
    Char { c: char },
    IntLiteral { raw: String },
}

pub struct Parser<'a> {
    _src: &'a str,
    pub tokens: Vec<crate::Token>,
    expect: Vec<crate::Terminal>,
    pub errors: Vec<ParserError>,
    lexer: crate::lexer::Lexer<'a>,
}
