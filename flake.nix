{
  description = "RARO: Running ARithmetic with Objects.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";

  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, ... }:
    {
      fenix = fenix;
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        lib = pkgs.lib;

        # Build just deps for caching, especially in CI
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
          strictDeps = true;
        };
      in
        {
          checks = {
            clippy = craneLib.cargoClippy {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            };
            doc = craneLib.cargoDoc {
              inherit cargoArtifacts;
            };
            fmt = craneLib.cargoFmt {
              inherit cargoArtifacts;
            };
          };
          packages.default = craneLib.buildPackage {
            inherit src;
          };
        }
    );
}
