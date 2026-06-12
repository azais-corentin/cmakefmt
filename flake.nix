{
  description = "Dev environment for cmakefmt";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      # Linux-only shell: google-chrome (unfree) and the LD_LIBRARY_PATH shim
      # are x86_64-linux specific.
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs [ "x86_64-linux" ] (
          system:
          f (
            import nixpkgs {
              inherit system;
              # google-chrome is unfree; nixpkgs.legacyPackages would not honor allowUnfree.
              config.allowUnfree = true;
              overlays = [ rust-overlay.overlays.default ];
            }
          )
        );
    in
    {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            # Rust stable + wasm target (replaces devenv languages.rust)
            (pkgs.rust-bin.stable.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [
                "rust-analyzer"
                "llvm-tools-preview"
              ];
            })
            pkgs.git
            pkgs.google-chrome
            pkgs.cargo-pgo
            pkgs.llvm
            # replaces devenv languages.python + uv
            pkgs.python3
            pkgs.uv
            # used by the hk nix-fmt pre-commit hook
            pkgs.nixfmt
          ];

          env = {
            # sharp native module needs libstdc++ on the linker path
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ];
            # Use Nix-packaged Chrome for Puppeteer (NixOS can't run Puppeteer's downloaded binary)
            PUPPETEER_SKIP_CHROMIUM_DOWNLOAD = "true";
            PUPPETEER_EXECUTABLE_PATH = "${pkgs.google-chrome}/bin/google-chrome";
          };
        };
      });
    };
}
