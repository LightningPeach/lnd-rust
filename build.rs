extern crate protoc_rust_grpc;

fn main() {
    println!("cargo:rerun-if-changed=protos/rpc.proto");

    let gopath = std::env::var("GOPATH").expect("GOPATH is not defined");
    let gopath = std::path::Path::new(&gopath);
    let google_api_path = gopath.join("src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis");
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["protos", google_api_path.to_str().unwrap()],
        input: &["protos/rpc.proto"],
        rust_protobuf: true, // also generate protobuf messages, not just services
    }).expect("protoc-rust-grpc");
}
