{
  description = "bang";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion =
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        bang = rustPlatform.buildRustPackage {
          pname = "bang";
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml
          cargoLock.lockFile = ./Cargo.lock;
        };

      in with pkgs; {
        defaultPackage = bang;
        devShell = mkShell {
          buildInputs = [
            (rustVersion.override { extensions = [ "rust-src" ]; })

            cargo-expand

            # common
            watchexec

            nixfmt
          ];
        };
      });
}
