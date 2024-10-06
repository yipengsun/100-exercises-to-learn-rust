{
  inputs = {
    nixpkgs-pointer.url = "github:yipengsun/nixpkgs-pointer";

    nixpkgs.follows = "nixpkgs-pointer/nixpkgs";
    flake-utils.follows = "nixpkgs-pointer/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          name = "100-exercises-to-learn-rust-dev";
          buildInputs = [
            openssl
            pkg-config
            rust-bin.stable.latest.default
          ];

          shellHook = ''
            export PATH=$HOME/.cargo/bin:$PATH
          '';
        };
      }
    );
}
