{
  description = "Boilermaker Rust dev shell with pkg-config support";

  inputs = {
    #nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; # comment out to use unstable
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # nativeBuildInputs is for tools you need to RUN (compilers, linkers)
          nativeBuildInputs = with pkgs; [
            rust-bin.stable.latest.default # or cargo/rustc if not using overlay
            pkg-config
          ];

          # buildInputs is for libraries you need to LINK against
          buildInputs = with pkgs; [
            atk
            glib
            openssl
            wayland
          ];

          # OPTIONAL: Useful for IDEs (rust-analyzer) to find libraries
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}:$LD_LIBRARY_PATH
          '';
        };
      }
    );
}
