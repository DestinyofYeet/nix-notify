{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    let
      overlays = [ (import inputs.rust-overlay) ];
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        inherit overlays;
      };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })

          rust-analyzer
          sqlite.dev
          openssl.dev
          pkg-config
        ];

        # uncomment this is you get some kind of ssl error, usually on anything networking related using reqwest
        # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };

      packages.x86_64-linux.default = pkgs.callPackage ./pkg.nix { };
    };
}
