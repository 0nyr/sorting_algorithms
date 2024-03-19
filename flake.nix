{
  description = "Prim's algorithm for finding minimum spanning trees";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = github:numtide/flake-utils;
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # development environment
        devShells.default = pkgs.mkShell {
          packages = [
            # Rust
            pkgs.cargo
            pkgs.rustc
            pkgs.clippy
            pkgs.rust-analyzer
            pkgs.rustup
            pkgs.rustfmt

            # LaTeX
            # LaTeX
            pkgs.texlive.combined.scheme-full
            pkgs.biber
            pkgs.gnumake
            
            # Adding Java for LTeX spell checker (vscode extension)
            pkgs.openjdk11
          ];

          # environment variables for Rust
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          # environment variables for Java
          JAVA_HOME = "${pkgs.openjdk11}/lib/openjdk";
          JAVA_OPTS = "-Xms64m -Xmx512m";
        };
      }
    );
}