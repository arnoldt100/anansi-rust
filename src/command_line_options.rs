/// The program command line options.

use std::path::PathBuf;
use clap::{Parser};

/// Processes the program command line options.
pub fn process_command_line_args() {

    println!("Processing command line arguments...");

    // For demonstration purposes, we will just return a dummy ProgramOptions 
    // struct.
    let cli = ProgramOptions::parse();
} 

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ProgramOptions {


    // Sets a custom command file name. This is a required argument that 
    // specifies the file path to the command file that contains the 
    // instructions for the molecular simulation.
    #[arg(short, long,help=_COMMAND_FILE_HELP_TEXT ,value_name = "COMMAND_FILE")]
    command_file: PathBuf,

    // Sets the debug level of the simulation. This is an optional argument 
    // that can be used to control the verbosity of the program's output.
    #[arg(long, help=_DEBUG_LEVEL_HELP_TEXT ,value_name = "DEBUG_LEVEL")]
    debug_level: Option<u8>,

    // Sets the restart flag. This is an optional argument that indicates 
    // whether the simulation should be restarted. If this flag is set to true, 
    // the program will attempt to restart previous simulation from a 
    // checkpoint. If it is set to false or not provided, the program will 
    // start a new simulation from the beginning.
    #[arg(long, help=_RESTART_FLAG_HELP_TEXT ,value_name = "RESTART_FLAG")]
    restart_flag: Option<bool>,
}

pub const _COMMAND_FILE_HELP_TEXT : &str = 
    "The file path to the molecular simulation command file.";
pub const _DEBUG_LEVEL_HELP_TEXT : &str = 
    "The debug level setting. See docs for information.";
pub const _RESTART_FLAG_HELP_TEXT : &str = 
    "The restart flag. Indicates whether to restart a previous simulation from a checkpoint.";
