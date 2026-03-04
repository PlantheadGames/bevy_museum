{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    fontconfig
  ];
  buildInputs = [
    libGL
    udev  vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  alsa-lib-with-plugins
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
    shellHook = ''
    export LD_LIBRARY_PATH="${lib.makeLibraryPath buildInputs}:${LD_LIBRARY_PATH}"
  '';
  NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [
    stdenv.cc.cc
    openssl.dev
    openssl
    # ...
  ];
  NIX_LD = lib.fileContents "${stdenv.cc}/nix-support/dynamic-linker";
}
