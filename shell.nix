{ pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
    name = "rust-shell";
    buildInputs = with pkgs; [
        cargo
        buildPackages.protobuf
            pkgs.nodePackages.webpack
            pkgs.nodePackages.webpack-cli
            pkgs.nodePackages.create-react-app
            openssl
            nodejs
            pkg-config
            docker-compose
            cmake
            rustup
            zlib
    ];
    hardeningDisable = ["fortify" ];
}
