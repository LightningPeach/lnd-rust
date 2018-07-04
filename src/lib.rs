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

pub struct TLSCertificate {
    raw: tls_api::Certificate,
}

impl TLSCertificate {
    pub fn from_der_path<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        use std::io::{Error, ErrorKind};

        let output = Command::new("openssl")
            .args(&["x509", "-outform", "der", "-in"])
            .arg(path.as_ref().as_os_str())
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(Error::new(ErrorKind::InvalidInput, error.as_ref()));
        }

        Ok(TLSCertificate {
            raw: tls_api::Certificate::from_der(output.stdout),
        })
    }

    pub fn create_client(
        self,
        socket_addr: &SocketAddr,
        host: &str,
        conf: grpc::ClientConf
    ) -> grpc::Client {
        let connector: tls_api_native_tls::TlsConnector = {
            let mut builder = tls_api_native_tls::TlsConnector::builder()
                .unwrap();
            builder
                .add_root_certificate(self.raw)
                .unwrap();
            builder
                .build()
                .unwrap()
        };

        let tls = httpbis::ClientTlsOption::Tls(host.to_owned(), Arc::new(connector));
        grpc::Client::new_expl(&socket_addr, host, tls, conf)
            .unwrap()
    }
}
