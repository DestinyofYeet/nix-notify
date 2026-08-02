{
  pkgs,
  ...
}:
{
  inputs = with pkgs; [

    sqlite.dev
    openssl.dev
    pkg-config
  ];
}
