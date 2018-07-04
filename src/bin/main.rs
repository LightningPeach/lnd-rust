extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;

extern crate lnd_rust;

use lnd_rust::rpc_grpc::Lightning;
use lnd_rust::TLSCertificate;

fn main() {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};

    println!("lnd-rust main");

    let default_path = "/home/twenty/work/ln-ico/simple-simnet/bitcoin-bitcoind/rpc/rpc.cert";
    let cert_filename = std::env::args()
        .into_iter().skip(1).next()
        .unwrap_or(default_path.to_owned());
    let certificate = TLSCertificate::from_der_path(cert_filename)
        .unwrap();

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10009);

    let grpc_client = certificate.create_client(&socket_addr, "localhost", Default::default());
    let client = lnd_rust::rpc_grpc::LightningClient::with_client(grpc_client);

    let req = lnd_rust::rpc::GetInfoRequest::new();
    let resp = client.get_info(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());

    let wallet_req = lnd_rust::rpc::WalletBalanceRequest::new();
    let wallet_resp = client.wallet_balance(grpc::RequestOptions::new(), wallet_req);
    let w = wallet_resp.wait().unwrap().1;
    println!("{:?}\n", w);
}
