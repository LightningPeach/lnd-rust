extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate bytes;

extern crate lnd_rust;

use lnd_rust::rpc_grpc::Lightning;
use lnd_rust::TLSCertificate;
use lnd_rust::MacaroonData;

fn main() {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};

    println!("lnd-rust main");

    let certificate = {
        let default_path = "/home/twenty/work/ln-ico/simple-simnet/bitcoin-bitcoind/rpc/rpc.cert";
        let cert_filename = std::env::args()
            .into_iter().skip(1).next()
            .unwrap_or(default_path.to_owned());
        TLSCertificate::from_der_path(cert_filename)
            .unwrap()
    };

    let client = {
        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10009);
        let grpc_client = certificate.create_client(&socket_addr, "localhost", Default::default());
        lnd_rust::rpc_grpc::LightningClient::with_client(grpc_client)
    };

    let macaroon_data = {
        let default_path = "tools/data/lnd1/admin.macaroon";
        let macaroon_file_path = std::env::args()
            .into_iter().skip(2).next()
            .unwrap_or(default_path.to_owned());
        MacaroonData::from_file_path(macaroon_file_path)
            .unwrap()
    };

    let req = lnd_rust::rpc::GetInfoRequest::new();
    let resp = client.get_info(macaroon_data.metadata(), req);
    println!("{:?}", resp.wait());

    let wallet_req = lnd_rust::rpc::WalletBalanceRequest::new();
    let wallet_resp = client.wallet_balance(macaroon_data.metadata(), wallet_req);
    let w = wallet_resp.wait().unwrap().1;
    println!("{:?}\n", w);
}
