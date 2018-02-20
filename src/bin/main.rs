extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate tls_api_native_tls;

extern crate lnd_rust;
use lnd_rust::rpc_grpc::Lightning;
use lnd_rust::{read_x509, create_grpc_client};

fn main(){
    println!("lnd-rust main");

    let certificate = read_x509("/home/twenty/work/ln-ico/simple-simnet/bitcoin/rpc/rpc.cert").unwrap();
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10009);

    let grpc_client = create_grpc_client(
        &socket_addr,
        "localhost",
        certificate,
        Default::default()
    );

    let client = lnd_rust::rpc_grpc::LightningClient::with_client(grpc_client);

    let req = lnd_rust::rpc::GetInfoRequest::new();
    let resp = client.get_info(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());
}
