#[cfg(test)]
mod main_progam_tests {
    #[test]
    fn test_arg_handling() {
        // We need to replicate passing CLI args to the function "load_file"
        // Create sub process to run the program
        let mut cmd;
        let mut output;
        let filename = "examples/hello_world.psc";

        cmd = std::process::Command::new("cargo");
        cmd.args(&["run", "--", format!("--src={}", filename).as_str()]);

        output = cmd.output().unwrap();
        // Check if the output contained "Unable to open file"
        if String::from_utf8(output.stderr)
            .unwrap()
            .contains("Unable to open file")
        {
            panic!("Unable to open file {}", filename);
        }

        cmd = std::process::Command::new("cargo");
        cmd.args(&["run", "--", filename]);
        output = cmd.output().unwrap();
        // Check if the output contained "Unable to open file"
        if String::from_utf8(output.stderr)
            .unwrap()
            .contains("Unable to open file")
        {
            panic!("Unable to open file {}", filename);
        }

        cmd = std::process::Command::new("cargo");
        cmd.args(&[
            "run",
            "--",
            "asdas",
            "asda",
            format!("--src={}", filename).as_str(),
            "hello",
            filename,
        ]);
        output = cmd.output().unwrap();
        // Check if the output contained "Unable to open file"
        if String::from_utf8(output.stderr)
            .unwrap()
            .contains("Unable to open file")
        {
            panic!("Unable to open file {}", filename);
        }
    }
}
