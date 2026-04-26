//! Introduction to anansi-rust
//!
//! ## Detailed introduction.
//!
//! ## Features
//!
//! ## Examples

mod command_line_options;
use command_line_options::process_command_line_args;
use command_line_options::_COMMAND_FILE_HELP_TEXT;

mod setup_simulation;
use setup_simulation::create_md_simulation;

mod run_simulation;
use run_simulation::run_md_simulation;

mod cleanup_simulation;
use cleanup_simulation::cleanup_md_simulation;

fn main() {
    // Process command line arguments.
    process_command_line_args();
    create_md_simulation();
    run_md_simulation();
    cleanup_md_simulation();
}
