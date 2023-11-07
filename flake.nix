{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [
          (import rust-overlay)
          (self: super: {
            rust-toolchain = super.rust-bin.stable."1.72.0".default;
          })
        ];
      in {
        devShells.default = pkgs.mkShellNoCC {
          packages = with pkgs; [
            docker-compose
            rust-toolchain
            cargo-edit
            cargo-nextest
            mysql
            sqlx-cli
          ];
          buildInputs = [ ];
          nativeBuildInputs = with pkgs;
            [ ] ++ lib.optionals stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [ SystemConfiguration ]);
        };
      });
}
