let
    moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    nixpkgs = import <nixpkgs> { overlays = [moz_overlay ]; };
    rustNightlyChannel = (nixpkgs.rustChannelOf { date = "2021-04-07"; channel = "nightly"; }).rust.override {
        extensions = [
            "rust-src"
        ];
    };
in
with nixpkgs;
pkgs.mkShell {
    name = "moz_overlay_shell";
    buildInputs = [
        pkgs.cargo
        buildPackages.protobuf
            rustc
            nodePackages.webpack
            nodePackages.webpack-cli
            rustfmt
            openssl
            nodejs
            pkg-config
            cmake
            rustNightlyChannel
            rustup
            zlib
    ];
    RUST_SRC_PATH = "${rustNightlyChannel}/lib/rustlib/src/rust/library";
    hardeningDisable = ["fortify" ];
}
