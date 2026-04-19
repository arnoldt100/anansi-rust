/// The program command line options.
pub struct ProgramOptions {
    command_file: String,
}

/// Processes the program command line options.
pub fn process_command_line_args() -> ProgramOptions{

    println!("Processing command line arguments...");

    // For demonstration purposes, we will just return a dummy ProgramOptions struct.
    ProgramOptions {
        command_file: String::from("commands.txt"),
    } 
}   


