{
  inputs,
  self,
  lib,
  repoRoot,
  ...
}:
let
  # List all toolchain files in `dir` (returns `[ "toolchain-go.nix" ... ]`  )
  toolchain-files = lib.pipe inputs.import-tree [
    # NOTE: Uncomment the below to inspect what toolchains are loaded.
    # (i: i.map (x: lib.info "Importing toolchain :${x}" x))
    (i: i.match ".*/toolchains/toolchain-.*\.nix")
    (i: i.withLib lib)
    (i: i.leafs repoRoot)
  ];

  # Imports all toolchain files into `{ "go" = [ list of devenv modules ], ... }`
  # Pass all `args` to the function.
  toolchain-import =
    args:
    let
      # Add inputs and lib to the arguments missing on `perSystem`.
      allArgs = args // {
        inherit inputs lib;
        inherit self;
      };
      imports = lib.map (path: import path allArgs) toolchain-files;
    in
    lib.attrsets.mergeAttrsList imports;
in
{
  import = toolchain-import;
}
