use anansi_rust;
fn main() {
    anansi_rust::process_command_line_args();
    anansi_rust::setup_simulation();
    anansi_rust::run_simulation();
    anansi_rust::cleanup_simulation();
}
