{ pkgs, inputs, ... }:

{
  packages = with pkgs; [
    git
    inputs.lintel.packages.${pkgs.system}.cargo-furnish
  ];

  languages.rust.enable = true;

  scripts.mad.exec = ''
    cargo run --release -p mad -- "$@"
  '';

  scripts.mad-debug.exec = ''
    cargo run -p mad -- "$@"
  '';

  git-hooks.hooks = {
    clippy = {
      enable = true;
      settings = {
        allFeatures = true;
        denyWarnings = true;
        extraArgs = "--all-targets";
      };
    };
    rustfmt.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
  };
}
