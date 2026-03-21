{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  # https://devenv.sh/basics/

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    google-chrome
    cargo-pgo
    llvm
  ];

  # sharp native module needs libstdc++ on the linker path
  env.LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ];

  # Use Nix-packaged Chromium for Puppeteer (NixOS can't run Puppeteer's downloaded binary)
  env.PUPPETEER_SKIP_CHROMIUM_DOWNLOAD = "true";
  env.PUPPETEER_EXECUTABLE_PATH = "${pkgs.google-chrome}/bin/google-chrome";

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "stable";
      targets = [ "wasm32-unknown-unknown" ];
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
        "llvm-tools-preview"
      ];
    };
    python = {
      enable = true;
      uv = {
        enable = true;
      };
    };
  };

  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/

  # https://devenv.sh/basics/

  # https://devenv.sh/tests/

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
