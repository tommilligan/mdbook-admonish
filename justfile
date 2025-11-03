set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`
flake_dir := root_dir / "tools/nix"
output_dir := root_dir / "target"
build_dir := output_dir / "build"

mod nix "./tools/just/nix.just"

# Default target if you do not specify a target.
default:
    just --list --unsorted

# Enter the default Nix development shell and execute the command `"$@`.
develop *args:
    just nix::develop "default" "$@"

# Build the project.
build *args:
    cargo build --target-dir "{{build_dir}}" "$@"

# Test the project.
test *args:
    cargo test "$@"

# Watch `cargo build|run|test ...` commands.
watch *args:
    cargo watch -x "$1" "${@:2}"

# Run an executable.
run *args:
    cargo run --target-dir "{{build_dir}}" "$@"

# Setup
setup:
    echo "Setting up project."
