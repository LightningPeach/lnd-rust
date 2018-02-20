extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate tls_api_native_tls;
extern crate httpbis;

pub mod rpc;
pub mod rpc_grpc;

use std::sync::Arc;

use std::process::Command;

use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;

pub fn read_x509(x509_file: &str) -> std::io::Result<tls_api::Certificate> {
    use std::io::{Error, ErrorKind};

    let output = Command::new("openssl")
                     .args(&["x509", "-outform", "der", "-in", x509_file])
                     .output()?;
    if !output.status.success() {
        return Err(Error::new(ErrorKind::InvalidInput, String::from_utf8_lossy(&output.stderr).as_ref()));
    }
    Ok(tls_api::Certificate::from_der(output.stdout))
}

pub fn create_grpc_client(socket_addr: &std::net::SocketAddr, host: &str, certificate: tls_api::Certificate, conf: grpc::ClientConf) -> grpc::Client {
    let mut tls_connector_builder = tls_api_native_tls::TlsConnector::builder()
        .unwrap();

    tls_connector_builder
        .add_root_certificate(certificate)
        .unwrap();

    let tls_connector = tls_connector_builder
        .build()
        .unwrap();

    let grpc_client = ::grpc::Client::new_expl::<tls_api_native_tls::TlsConnector>(
        &socket_addr,
        host,
        httpbis::ClientTlsOption::Tls(host.to_owned(), Arc::new(tls_connector)),
        conf
    ).unwrap();
    grpc_client
}