set positional-arguments
set shell := ["bash", "-cue"]

default:
    just --list --unsorted -f "{{source_file()}}"

# Enter the Nix development shell `$1` and execute the command `${@:2}`.
[no-cd]
develop *args:
    #!/usr/bin/env bash
    set -eu
    root_dir=$(git rev-parse --show-toplevel) || exit 1
    flake_dir="$root_dir/tools/nix"

    shell="$1"; shift 1;
    args=("$@") && [ "${#args[@]}" != 0 ] ||
        args=(env SHELL="$SHELL" "$SHELL")

    nix_args=("--no-pure-eval")

    nix develop \
        "${nix_args[@]}" \
        --accept-flake-config \
        "$flake_dir#$shell" \
        --command "${args[@]}"

# Start an Nix interpreter with loaded flake.
[no-cd]
repl *args:
    #!/usr/bin/env bash
    set -eu

    root_dir=$(git rev-parse --show-toplevel) || exit 1
    flake_dir="$root_dir/tools/nix"
    nix_args=("--no-pure-eval" "$@")

    nix repl \
        "${nix_args[@]}" \
        --accept-flake-config \
        "$flake_dir"

# Build a package in the `packages` output of `flake.nix`
alias build := package
[no-cd]
package attrname *args:
    #!/usr/bin/env bash
    set -eu
    root_dir=$(git rev-parse --show-toplevel) || exit 1
    flake_dir="$root_dir/tools/nix"
    nix_args=("${@:2}")

    mkdir -p .output/package
    outlink=".output/package/{{attrname}}"
    nix build \
        --accept-flake-config \
        "${nix_args[@]}" \
        -L "$flake_dir#{{attrname}}" \
        --out-link "$outlink" --json

# Show all packages.
[no-cd]
show:
    #!/usr/bin/env bash
    set -eu
    root_dir=$(git rev-parse --show-toplevel) || exit 1
    flake_dir="$root_dir/tools/nix"

    # We cannot use `nix flake show` due to other systems where
    # IFD (import-from-derivation -> from-yaml Nix derivation)
    # does not work.
    system=$(nix eval --impure --raw --expr builtins.currentSystem)
    nix eval --json "$flake_dir#packages.$system" | jq
