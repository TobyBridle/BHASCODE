pub mod lexer;
pub use lexer::*;
pub mod parser;
pub use parser::*;

// Main Program Error
#[macro_export]
macro_rules! main_program_err {
( $( $str: expr ),* ) => {
        {
            eprintln!("\x1b[31;1m[Main] Error: {}\x1b[0;0m", $( $str ),*);
        }
    };
}

// Main Program Warn
#[macro_export]
macro_rules! main_program_warn {
( $( $str: expr ),* ) => {
        {
            eprintln!("\x1b[33;1m[Main] Warn: {}\x1b[0;0m", $( $str ),*);
        }
    };
}

// Generic Program Error
#[macro_export]
macro_rules! program_error {
( $( $str: expr, $tag: expr ),* ) => {
        {
            eprintln!("\x1b[31;1m[{1}] Error: {0}\x1b[0;0m", $( $str, $tag),*);
        }
    };
}

// Generic Program Warn
#[macro_export]
macro_rules! program_warn {
( $( $str: expr, $tag: expr ),* ) => {
        {
            eprintln!("\x1b[33;1m[{1}] Warn: {0}\x1b[0;0m", $( $str, $tag),*);
        }
    };
}

// Generic Program Success
#[macro_export]
macro_rules! program_success {
( $( $str: expr, $tag: expr ),* ) => {
        {
            println!("\x1b[32;1m[{1}] Success: {0}\x1b[0;0m", $( $str, $tag),*);
        }
    };
}
