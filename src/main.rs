extern crate bhascode;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time;

use bhascode::*;

/// # Parse the file handle and retrieve the data of the file.
/// File handle can be passed using `--src=<FILE>` or by passing as the first argument to the program
/// (e.g ./main `<FILE>`)
///
/// * `args`:
fn load_file(args: std::env::Args) -> Result<(std::fs::File, String), Box<dyn std::error::Error>> {
    if args.len() == 0 {
        main_program_err!("No source file provided");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No source file provided",
        )));
    }

    // Parse the file-handle. It may have been passed as --src=<FILE> or simply just <FILE>
    // If it was passed as --src=<FILE> then we need to find the index of the parameter and use
    // that
    // If it was not passed like that, we can assume the first arg as the file handle
    let mut file_handle = String::new();
    let mut fallback_handle = String::new();
    args.enumerate().for_each(|(i, arg)| {
        if i == 1 {
            fallback_handle = arg.clone();
        }

        // If the arg starts with --src= then we know it's the file handle
        if arg.starts_with("--src=") {
            file_handle = arg.replace("--src=", "");
        }
    });

    if file_handle.is_empty() {
        // We want to check if the fallback_handle is a string rather than a file handle
        // It may be that the user wants to parse the string rather than a file.
        if (fallback_handle.starts_with('"') && fallback_handle.ends_with('"'))
            || (fallback_handle.starts_with('\'') && fallback_handle.ends_with('\''))
        {
            // Create virtual file with the data inside the string and file_handle of "Inline
            // Program"
            // The type of File must be std::fs::File
            // Create a unique file name using the time
            let time = time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            // Set an environment variable to the file name
            std::env::set_var("BHASCODE_TEMP_FILE", format!("inline_program_{}.psc", time));
            let mut file = File::create(format!("inline_program_{}.psc", time))?;
            // Write data asynchronously without quotes
            file.write_all(fallback_handle[1..fallback_handle.len() - 1].as_bytes())?;
            // Close the file
            std::mem::drop(file);
            file = File::open(format!("inline_program_{}.psc", time))?;
            return Ok((file, "Inline Program".to_string()));
        } else {
            file_handle = fallback_handle;
        }
    }

    let file = std::fs::File::open(file_handle.clone());
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

    Ok((file.unwrap(), file_handle.to_string()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file from cli args e.g (./main --src=examples/hello_world.psc) or simply (./main
    // examples/hello_world.psc)

    let (mut file, file_handle) = load_file(std::env::args())?;

    let mut src = String::new();
    let _ = file.read_to_string(&mut src);

    // Warn the user that the file is empty
    if src.is_empty() {
        main_program_warn!(format!(
            "File {} is empty, are you sure that the correct file was passed?",
            file_handle
        ));
    }

    let mut parser = Parser::new(&src);
    parser.parse();

    // Print all of the tokens that were found
    parser.tokens.iter().for_each(|token| {
        println!("{}", token);
    });

    // Recurse errors (if any) and print using macro
    parser.errors.iter().for_each(|err| {
        program_error!(format!("{}", err), "Parser");
    });

    // Delete the virtual file if it was created
    if std::env::var("BHASCODE_TEMP_FILE").is_ok() {
        std::fs::remove_file(std::env::var("BHASCODE_TEMP_FILE").unwrap())?;
    }

    Ok(())
}
