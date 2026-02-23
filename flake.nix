{
  description = "mad - A fast Markdown terminal renderer with syntax highlighting";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        src =
          let
            inherit (pkgs) lib;
            readmeFilter = path: _type: (builtins.match ".*README\\.md$" path) != null;
            syntaxFilter = path: _type: (builtins.match ".*\\.sublime-syntax$" path) != null;
          in
          lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              (craneLib.filterCargoSources path type) || (readmeFilter path type) || (syntaxFilter path type);
          };

        packages' = import ./nix/packages.nix { inherit craneLib pkgs src; };
        inherit (packages') mad;
      in
      {
        checks = {
          inherit mad;
        };

        packages = {
          inherit mad;
          default = mad;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = mad;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
        };
      }
    );
}
