{
  rustPlatform,
  lib,
  pkgs,
  toml,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = toml.package.name;
  version = toml.package.version;

  src = ../.;

  cargoHash = "sha256-iuPMXumJdsVto8MV9UNGwzrKANvop8PEba/dhmh5UpE=";

  buildInputs = (import ./deps.nix { inherit pkgs; }).inputs;
  nativeBuildInputs = buildInputs;

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  meta = with lib; {
    mainProgram = toml.package.name;
    description = "A program";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
