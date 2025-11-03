{
  description = "repository-template";

  nixConfig = {
    extra-trusted-substituters = [
      # Nix community's cache server
      "https://nix-community.cachix.org"
      "https://devenv.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
    ];
  };

  # We use flake-parts to assemble all flake outputs.
  # This gives nicer modularity. All `.parts` files are
  # `flake-parts` module files.
  outputs =
    inputs:
    let
      lib = inputs.nixpkgs.lib;
    in
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      (
        lib.pipe inputs.import-tree [
          # NOTE: Uncomment the below to inspect what modules are loaded.
          # (i: i.map (x: lib.info "Importing :${x}" x))
          (i: i.filter (lib.hasInfix ".parts."))
          (i: i ./.)
        ]
      );

  inputs = {
    # Importing flake-parts modules recursively.
    import-tree = {
      url = "github:vic/import-tree";
    };

    systems = {
      # Using `nix-systems` flake specification.
      url = "path:./flake/systems.nix";
      flake = false;
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };

    # Nixpkgs (latest ones).
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Nixpkgs (stable NixOS branch)
    nixpkgs-stable.url = "github:nixos/nixpkgs/nixos-25.05";

    # Format the repo with nix-treefmt.
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # The devenv module to create good development shells.
    # The `nixpkgs-devenv` must aligned with the pinned version.
    devenv = {
      url = "github:cachix/devenv?ref=v1.8.1";
      inputs.nixpkgs.follows = "nixpkgs-devenv";
    };
    # This is the rolling nixpkgs with what devenv was tested.
    nixpkgs-devenv = {
      url = "github:cachix/devenv-nixpkgs?ref=0ceffe312871b443929ff3006960d29b120dc627";
    };

    # To build a base image with Nix.
    nix = {
      url = "github:nixos/nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    
    # The Rust overlay to include the latest toolchain.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    
  };
}
