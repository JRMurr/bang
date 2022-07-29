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
        #   pkgs.rust-bin.stable.latest.default;
      in with pkgs; {

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
