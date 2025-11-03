{
  self,
  inputs,
  lib,
  ...
}:
{
  perSystem =
    {
      config,
      ...
    }:
    let
      args = config.allModuleArgs; # See https://flake.parts/module-arguments#obtaining-all-module-arguments
      toolchains = self.lib.toolchain.import args;
      language = "rust";
    in
    {
      devShells.default = self.lib.shell.mkShell {
        inherit (args) system;
        modules = toolchains.${language};
      };
    };
}
