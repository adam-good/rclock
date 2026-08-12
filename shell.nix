{ pkgs ? import <nixpkgs> {}} :
pkgs.mkShellNoCC {
    packages = with pkgs; [
        gcc
        cargo
        rustc
        rustfmt
        rust-analyzer
    ];
}
