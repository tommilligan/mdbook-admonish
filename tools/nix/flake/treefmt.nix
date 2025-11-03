{ pkgs, ... }:
{
  # Used to find the project root
  projectRootFile = ".git/config";

  settings.global.excludes = [ "external/*" ];

  # Markdown, JSON, YAML, etc.
  programs.prettier.enable = true;

  # Python
  programs.ruff.enable = true;

  # Shell.
  programs.shfmt = {
    enable = true;
    indent_size = 4;
  };
  programs.shellcheck.enable = true;

  programs.gofmt.enable = true;
  programs.goimports.enable = true;

  # Lua.
  programs.stylua.enable = true;

  # Nix.
  programs.nixfmt.enable = true;

  # Typos.
  programs.typos.enable = false;
}
