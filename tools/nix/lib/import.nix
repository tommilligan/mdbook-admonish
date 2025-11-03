{
  inputs,
  ...
}:
let
  config = {
    allowUnfree = true;
  };
in
{
  pkgs =
    {
      system,
      overlays ? [ ],
    }:
    import inputs.nixpkgs {
      inherit system overlays config;
    };
  pkgsStable =
    {
      system,
      overlays ? [ ],
    }:
    import inputs.nixpkgs-stable {
      inherit system overlays config;
    };
}
