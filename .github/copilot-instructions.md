# Project Guidelines

## Code Style
- Keep Rust changes minimal and consistent with existing patterns in `src/lib.rs` and `src/main.rs`.
- Expose callable workflow steps as public functions in `src/lib.rs`; call them from `src/main.rs`.
- Match current output style (`println!` status messages) when extending simulation steps.
- Do not introduce new formatting/lint tools unless requested; no repo lint config is currently defined.

## Architecture
- This is a single Rust crate configured in `Cargo.toml` with:
  - library target in `src/lib.rs`
  - binary entrypoint in `src/main.rs`
- Current binary execution is an ordered pipeline:
  1. `process_command_line_args`
  2. `setup_simulation`
  3. `run_simulation`
  4. `cleanup_simulation`
- Documentation is generated with Sphinx:
  - source: `sphinx_root/source/`
  - generated output: `docs/`
  - config: `sphinx_root/source/conf.py`

## Build and Test
- Rust build: `cargo build`
- Rust run: `cargo run`
- Rust tests: `cargo test`
- Build docs (preferred): `bash scripts/anansi_sphinx_build.sh`
- Equivalent docs command: `uv run sphinx-build -M html ./sphinx_root/source ./docs`

## Project Conventions
- Prefer adding user-facing function docs under `sphinx_root/source/usage/` and linking them in `sphinx_root/source/index.rst` toctree.
- The docs stack uses both `.rst` and Markdown (`myst_parser` enabled in `sphinx_root/source/conf.py`).
- Keep repository layout stable: Rust code in `src/`, docs tooling in `scripts/`, Sphinx project in `sphinx_root/`.

## Integration Points
- Rust dependencies: none currently listed in `Cargo.toml` `[dependencies]`.
- Python/doc dependencies are declared in `pyproject.toml` and include:
  - `sphinx==8.2.3`
  - `myst-parser>=4.0.1`
  - `sphinxcontrib-rust==1.0.1`
- Docs build scripts rely on `uv` for dependency execution (`scripts/anansi_sphinx_build.sh`).

## Security
- No auth, secrets, or network integration patterns are currently discoverable in `src/` or `scripts/`.
- If adding external I/O, document inputs/outputs and avoid logging sensitive values by default.
