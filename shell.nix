{ pkgs ? import <nixpkgs> {} }:
(pkgs.mkShell {
  name = "rust-env";
  nativeBuildInputs = with pkgs; [
      cargo
      rustc
      rustup
	  rustfmt
      trunk
      cargo-binutils
      lld
	  openssl
	  pkg-config
  ];
})
