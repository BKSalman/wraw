{
  description = "basic rust development evnvironment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {nixpkgs, rust-overlay, ...}:
      let 
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
      in
    with pkgs; {
      devShells.${system}.default = mkShell {

          packages = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            })
            trunk
          ];
          
          nativeBuildInputs = [ ];
          
          buildInputs = [
            pkg-config
            openssl
          ];
        };

      formatter.x86_64-linux = legacyPackages.${system}.nixpkgs-fmt;
    };
}

