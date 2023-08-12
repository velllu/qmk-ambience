 { pkgs ? import <nixpkgs> { } }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    pkg-config
    hidapi
    libusb1
    xorg.libX11.dev
    xorg.libX11
  ];

  buildInputs = [
    hidapi
    libusb1
    xorg.libX11.dev
    xorg.libX11
  ];
}