{
  description = "Rust Service DevShell";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {

        devShells.default = mkShell {
          buildInputs = [
            # Rust build requirements
            openssl
            pkg-config

            # Rust and LSP
            rust-bin.stable.latest.default
            rust-analyzer

            # Other
            docker-compose
            just
            sqlx-cli
          ];

          shellHook = ''
          '';
        };
      }
    );
}
