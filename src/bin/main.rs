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
    use std::net::SocketAddr;

    println!("lnd-rust main");

    if std::env::args().len() < 4 {
        println!("Usage: cargo run -- %path_to_cert% %path_to_macaroon% %socket e.g. 127.0.0.1:100500%");
        return
    }

    let certificate = {
        let cert_filename = std::env::args()
            .into_iter().skip(1).next()
            .unwrap();
        TLSCertificate::from_der_path(cert_filename)
            .unwrap()
    };

    let macaroon_data = {
        let macaroon_file_path = std::env::args()
            .into_iter().skip(2).next()
            .unwrap();
        MacaroonData::from_file_path(macaroon_file_path)
            .unwrap()
    };

    let client = {
        let default = "127.0.0.1:10009";
        let socket_addr_string = std::env::args()
            .into_iter().skip(3).next()
            .unwrap_or(default.to_owned());
        let socket_addr: SocketAddr = socket_addr_string.parse().unwrap();
        let host = socket_addr.ip().to_string();
        let grpc_client = certificate.create_client(&socket_addr, host.as_str(), Default::default());
        lnd_rust::rpc_grpc::LightningClient::with_client(grpc_client)
    };

    let metadata = |macaroon_data: &MacaroonData|
        grpc::RequestOptions { metadata: macaroon_data.metadata(), };

    let req = lnd_rust::rpc::GetInfoRequest::new();
    let resp = client.get_info(metadata(&macaroon_data), req);
    println!("{:?}", resp.wait());

    let wallet_req = lnd_rust::rpc::WalletBalanceRequest::new();
    let wallet_resp = client.wallet_balance(metadata(&macaroon_data), wallet_req);
    let w = wallet_resp.wait().unwrap().1;
    println!("{:?}\n", w);
}
