extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/proto/grpc";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(&["api.proto"], &[proto_root], &proto_root, None)
        .expect("failed to compile gRPC definitions!");
}
