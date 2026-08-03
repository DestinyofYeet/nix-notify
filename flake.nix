{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:

      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import nixpkgs {
          inherit overlays system;
        };

        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            with pkgs;
            [
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
              })

              rust-analyzer
            ]
            ++ (import ./nix/deps.nix { inherit pkgs; }).inputs;

          # uncomment this is you get some kind of ssl error, usually on anything networking related using reqwest
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        packages.default = pkgs.callPackage ./nix/pkg.nix { inherit toml; };

        nixosModules.default = import ./nix/options.nix {
          flake = self;
          inherit toml;
          inherit system;
        };
      }
    );
}
