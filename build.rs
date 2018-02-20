extern crate protoc_rust_grpc;

fn main() {
    let gopath = std::env::var("GOPATH").expect("GOPATH is not defined");
    let gopath = std::path::Path::new(&gopath);
    let google_api_path = gopath.join("src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis");
    println!("google api path: {:?}", google_api_path);
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["protos", google_api_path.to_str().unwrap()],
        input: &["protos/rpc.proto"],
        rust_protobuf: true, // also generate protobuf messages, not just services
    }).expect("protoc-rust-grpc");
}
