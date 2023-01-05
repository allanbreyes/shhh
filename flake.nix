{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in {
        devShells.default = let
          frameworks = pkgs.darwin.apple_sdk.frameworks;
        in pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            gcc
            rustc
          ];
          buildInputs = with pkgs; [
            cargo-watch
            clippy
            openssl
            pkg-config
            rust-analyzer
            rustfmt
          ] ++ lib.optional stdenv.isDarwin [
            libiconv
            frameworks.Security
            frameworks.CoreFoundation
            frameworks.CoreServices
          ];

          shellHook = (
            if pkgs.stdenv.isDarwin then
              ''
                export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation -F${frameworks.Security}/Library/Frameworks -framework Security $NIX_LDFLAGS";
              ''
            else
              ""
          );

          RUST_BACKTRACE = 1;
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
