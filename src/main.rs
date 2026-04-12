mod command_line_options;
mod setup_simulation;
mod run_simulation;
mod cleanup_simulation;

fn main() {
    command_line_options::process_command_line_args();
    setup_simulation::create_md_simulation();
    run_simulation::run_md_simulation();
    cleanup_simulation::cleanup_md_simulation();
}
