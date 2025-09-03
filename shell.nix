{ pkgs ? import <nixpkgs> {} }:
(pkgs.mkShell {
  name = "rust-env";
  nativeBuildInputs = with pkgs; [
      cargo
      rustc
      rustup
      trunk
      cargo-binutils
      lld
  ];
})
