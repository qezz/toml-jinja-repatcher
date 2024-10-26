let
  rust_overlay = import (
    builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"
  );
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };

  rustVersion = "1.76.0";

  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };
in
pkgs.mkShell rec {
  buildInputs = with pkgs; [ rust ];

  RUST_BACKTRACE = 1;
}
