{
  self,
  inputs,
  lib,
  ...
}:
let
  repoRoot = ./../../..;

  # Lib filesystem.
  libFS = {
    # The repository root directory (inside the Nix store).
    inherit repoRoot;

    # The repopsitory root fileset to be used with the
    # `lib.fileset` library.
    repoRootFileset = lib.fileset.fromSource repoRoot;
  };

  # Lib import.
  libImport = import ./import.nix { inherit inputs self; };

  # Lib shell.
  libShell = import ./shell.nix { inherit inputs lib; };

  # Lib toolchains.
  libToolchain = import ./toolchains.nix {
    inherit
      repoRoot
      inputs
      self
      lib
      ;
  };
in
{
  flake.lib = {
    import = libImport;
    shell = libShell;
    fs = libFS;
    toolchain = libToolchain;
  };
}
