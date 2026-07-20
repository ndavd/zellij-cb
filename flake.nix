{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      rust-overlay,
      treefmt-nix,
      ...
    }:
    let
      systems = [
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      target = "wasm32-wasip1";
      forEachSystem = nixpkgs.lib.genAttrs systems;
      perSystem = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ target ];
            extensions = [ "clippy" ];
          };
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;
            CARGO_BUILD_TARGET = target;
            doCheck = false;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        in
        {
          package = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
          devShell = craneLib.devShell { };
          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );
          fmt = treefmtEval.config.build.check self;
          formatter = treefmtEval.config.build.wrapper;
        }
      );
    in
    {
      packages = forEachSystem (system: {
        default = perSystem.${system}.package;
      });
      devShells = forEachSystem (system: {
        default = perSystem.${system}.devShell;
      });
      checks = forEachSystem (system: {
        inherit (perSystem.${system}) package clippy fmt;
      });
      formatter = forEachSystem (system: perSystem.${system}.formatter);
    };
}
