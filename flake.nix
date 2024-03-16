{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      devShells.default = pkgs.mkShell {
        buildInputs =
          (with pkgs; [nodejs])
          ++ packages.palachias.buildInputs
          ++ packages.palachias.nativeBuildInputs;
        RUSTC_VERSION = "stable";
        # https://github.com/rust-lang/rust-bindgen#environment-variables
        shellHook = ''
          export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
        '';
      };

      formatter = pkgs.alejandra;

      packages = {
        palachias = pkgs.rustPlatform.buildRustPackage {
          pname = "palachias";
          version = "0.1.0";
          src = ./.;
          cargoHash = "sha256-ZD/2JHZLKg37ka//Mz0d2hf9PktqR6DWOUwn4n05JuY=";
          buildInputs = [pkgs.openssl];
          nativeBuildInputs = [pkgs.pkg-config];
        };
      };
    });
}
