extern crate httpbis;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate tls_api_native_tls;

use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;

use std::io::Read;

extern crate lnd_rust;
use lnd_rust::rpc_grpc::Lightning;

use std::sync::Arc;

fn main(){
    println!("lnd-rust main");

    let certificate = {
        let mut file = std::fs::File::open("cert.der").unwrap();
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        tls_api::Certificate::from_der(contents)
    };

    let mut tls_connector_builder = tls_api_native_tls::TlsConnector::builder()
        .unwrap();

    tls_connector_builder
        .add_root_certificate(certificate)
        .unwrap();

    let tls_connector = tls_connector_builder
        .build()
        .unwrap();

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10009);

    let grpc_client = ::grpc::Client::new_expl::<tls_api_native_tls::TlsConnector>(
        &socket_addr,
        "localhost",
        httpbis::ClientTlsOption::Tls("localhost".to_owned(), Arc::new(tls_connector)),
        Default::default()
    ).unwrap();

    let client = lnd_rust::rpc_grpc::LightningClient::with_client(grpc_client);

    let req = lnd_rust::rpc::GetInfoRequest::new();
    let resp = client.get_info(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());
}
