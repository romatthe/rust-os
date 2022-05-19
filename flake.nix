{
  description = "Bare metal pure-Rust OS based on Philipp Oppermann's blog posts";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-21.11";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # rust-version = "1.60.0";
        # rust-stable = pkgs.rust-bin.stable.${rust-version}.default.override {
        rust-stable = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ 
            "x86_64-unknown-linux-gnu"
            "thumbv7em-none-eabihf"
          ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-stable
            rust-analyzer
          ];
        };
      }
    );
}
