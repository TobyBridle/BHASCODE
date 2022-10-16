pub mod lexer;
pub use lexer::*;

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
