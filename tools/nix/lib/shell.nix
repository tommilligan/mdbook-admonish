{ inputs, lib, ... }:
{
  # Make a devenv shell (from `inputs.devenv`) with the following features:
  # - Using `pkgs` or imported from flake input `inputs.nixpkgs-devenv` for the `system` if not given.
  # - Additional `modules` added.
  # - Allow unfree packages.
  # - Flake integration set to `true`
  # - Misc: If `inputs.devenv-root` is given then `devenv.root` is set to the flake input `inputs.devenv-root`
  #   defined by
  #
  #    ```nix
  #       devenv-root = {
  #         url = "file+file:///dev/null";
  #         flake = false;
  #       };
  #     ```
  #   and set with `nix develop --no-pure-eval --override-input devenv-root "path:.devenv/state/pwd"
  #   where `.devenv/state/pwd` is the current project root directory. This is a workaround to allow
  #   pure evaluation.
  mkShell =
    {
      modules ? [ ],
      pkgs ? null,
      system ? null,
    }:
    assert lib.assertMsg (lib.hasAttr "self" inputs) "Inputs must contain `self`.";
    assert lib.assertMsg (lib.hasAttr "devenv" inputs) "Inputs must contain `devenv`.";
    assert lib.assertMsg (lib.hasAttr "nixpkgs-devenv" inputs) "Inputs must contain `nixpkgs-devenv`.";
    let
      pkgsForDevenv =
        if pkgs == null then
          assert lib.assertMsg (system != null) "System must be given";
          import inputs.nixpkgs-devenv {
            config.allowUnfree = true;
            inherit system;
          }
        else
          pkgs;

      # Only inject what devenv really uses;
      ins = {
        inherit (inputs) self devenv;

        # Why nixpkgs is used is a bit weird.
        # See https://github.com/cachix/devenv/pull/2091
        nixpkgs = inputs.nixpkgs-devenv;
      };

    in
    inputs.devenv.lib.mkShell {
      inputs = ins;
      pkgs = pkgsForDevenv;

      modules = [
        (
          {
            devenv.flakesIntegration = true;
          }
          # Only apply it if `devenv-root` is defined.
          // lib.optionalAttrs (lib.hasAttr "devenv-root" inputs) {
            # This is currently needed for devenv to properly run in pure hermetic
            # mode while still being able to run processes & services and modify
            # (some parts) of the active shell.
            # We read here the root for devenv from the workaround flake input `devenv-root`.
            devenv.root = lib.strings.trim (builtins.readFile inputs.devenv-root.outPath);
          }
        )
      ]
      ++ modules;
    };
}
