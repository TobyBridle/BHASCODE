extern crate bhascode;
use std::io::Read;

use bhascode::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file from cli args e.g (./main --src=examples/hello_world.psc) or simply (./main
    // examples/hello_world.psc)
    let file_handle = std::env::args().nth(1);
    if file_handle.is_none() {
        main_program_err!("No source file provided");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No source file provided",
        )));
    }

    // Unwrap the file handle as we now know it exists
    let ref file_handle = file_handle.unwrap();

    let file = std::fs::File::open(file_handle);
    if file.is_err() {
        main_program_err!(format!(
            "Unable to open file: {}. Make sure that it exists first",
            file_handle
        ));
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to open file",
        )));
    }

    let mut src = String::new();
    let _ = file.unwrap().read_to_string(&mut src); // Result stored in temporary var

    // Warn the user that the file is empty
    if src.is_empty() {
        main_program_warn!(format!(
            "File {} is empty, are you sure that the correct file was passed?",
            file_handle
        ));
    }

    let mut lexer = Lexer::new(&src);
    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(tok) => println!("{:?}", tok),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
