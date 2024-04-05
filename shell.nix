{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      rustup
      git
    ];
    shellHook = ''
    rustup default stable
    rustup component add rust-analyzer
    '';
  }
