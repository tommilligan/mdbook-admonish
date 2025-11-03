# This function returns a attrset of `devenv` modules
# which can be passed to `mkShell`.
{
  self',
  pkgs,
  ...
}:
{
  rust = [
    {
      packages = [
        self'.packages.rust-toolchain
        pkgs.cargo-watch

        # Debugging
        pkgs.lldb_18
      ];
      enterShell = ''
        repo_dir=$(git rev-parse --show-toplevel)
        # Set the default output directory.
        export CARGO_TARGET_DIR="$repo_dir/build"
        unset repo_dir

        just setup
      '';
    }
  ];
}
