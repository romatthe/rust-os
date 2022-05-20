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
        pkgs = import nixpkgs { inherit system overlays; };
        # rust-version = "1.60.0";
        # rust-stable = pkgs.rust-bin.stable.${rust-version}.default.override {
        rust-dist = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "llvm-tools-preview" "rust-src" "rustfmt" ];
          targets = [ "x86_64-unknown-linux-gnu" "thumbv7em-none-eabihf" ];
        };
        cargo-bootimage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-bootimage";
          version = "0.10.3";

          src = pkgs.fetchFromGitHub {
            owner = "rust-osdev";
            repo = "bootimage";
            rev = "v${version}";
            sha256 = "sha256-74/aaZR+KHlZjZRuxeXjaLKjja1/7OaHjuMQOmZF4Yo=";
          };

          cargoSha256 = "sha256-9rZ42JXmn5IyYlFgfawwBR2/XUBQWOLCN+sSnVOoqgE=";
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo-bootimage
            qemu
            nixfmt
            rust-analyzer
            rust-dist
          ];
        };
      });
}
