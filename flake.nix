{
  description = "yaru flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        rust-overlay.follows = "rust-overlay";
      };
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Tell crane telemetryd Cargo.toml path to inspect package name and version
        yaruCrate = craneLib.crateNameFromCargoToml { cargoToml = ./telemetryd/Cargo.toml; };

        yaru = craneLib.buildPackage {
          inherit (yaruCrate) pname version;
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          cargoExtraArgs = "--package yaru";

          buildInputs = [ ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ ];
        }; 
      in {
        packages.default = self.packages."${system}".yaru;
        packages.yaru = yaru;
      });
}
