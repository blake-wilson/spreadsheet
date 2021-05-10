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
            nodePackages.webpack
            nodePackages.webpack-cli
            nodePackages.create-react-app
            openssl
            nodejs
            pkg-config
            cmake
            rustup
            zlib
    ];
    hardeningDisable = ["fortify" ];
}
