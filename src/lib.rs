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

use std::convert::AsRef;
use std::path::Path;
use std::io::Result as IOResult;

use std::net::SocketAddr;

pub fn read_x509<P: AsRef<Path>>(x509_file: P) -> IOResult<tls_api::Certificate> {
    use std::io::{Error, ErrorKind};

    let output = Command::new("openssl")
        .args(&["x509", "-outform", "der", "-in"])
        .arg(x509_file.as_ref().as_os_str())
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(ErrorKind::InvalidInput, error.as_ref()));
    }
    Ok(tls_api::Certificate::from_der(output.stdout))
}

pub fn create_grpc_client(
    socket_addr: &SocketAddr,
    host: &str,
    certificate: tls_api::Certificate,
    conf: grpc::ClientConf
) -> grpc::Client {
    let tls_connector: tls_api_native_tls::TlsConnector = {
        let mut tls_connector_builder = tls_api_native_tls::TlsConnector::builder()
            .unwrap();
        tls_connector_builder
            .add_root_certificate(certificate)
            .unwrap();
        tls_connector_builder
            .build()
            .unwrap()
    };

    let tls = httpbis::ClientTlsOption::Tls(host.to_owned(), Arc::new(tls_connector));
    ::grpc::Client::new_expl(&socket_addr, host, tls, conf)
        .unwrap()
}
