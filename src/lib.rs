extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate tls_api_native_tls;
extern crate httpbis;
extern crate bytes;

pub mod rpc;
pub mod rpc_grpc;

use std::sync::Arc;
use std::net::SocketAddr;
use std::process::Command;

use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;

use std::convert::AsRef;
use std::path::Path;
use std::io::Result as IOResult;

/// Represents bytes of the certificate
/// could be used to create `grpc::Client`
pub struct TLSCertificate {
    raw: tls_api::Certificate,
}

impl TLSCertificate {
    /// Reads the certificate in the der format from a file at the path
    pub fn from_der_path<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        use std::io::{Error, ErrorKind};

        let output = Command::new("openssl")
            .args(&["x509", "-outform", "der", "-in"])
            .arg(path.as_ref())
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(Error::new(ErrorKind::InvalidInput, error.as_ref()));
        }

        Ok(TLSCertificate {
            raw: tls_api::Certificate::from_der(output.stdout),
        })
    }

    /// Creates the `grpc::Client` using this certificate
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

/// Represents the bytes of the macaroon
pub struct MacaroonData {
    raw: Vec<u8>,
}

impl MacaroonData {
    /// Reads the macaroon data from a file at the path
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        use std::io::{Error, ErrorKind};

        let output = Command::new("xxd")
            .args(&["-ps", "-u", "-c", "1000"])
            .arg(path.as_ref())
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(Error::new(ErrorKind::InvalidInput, error.as_ref()))
        } else {
            let mut data = output.stdout;
            data.retain(|&z|
                ((z >= '0' as _) && (z <= '9' as _)) |
                    ((z >= 'A' as _) && (z <= 'F' as _))
            );
            Ok(MacaroonData {
                raw: data,
            })
        }
    }

    /// Creates the `grpc::Metadata` instance that contain the provided macaroon
    pub fn metadata(&self) -> grpc::Metadata {
        let macaroon = bytes::Bytes::from(self.raw.clone());
        let mut metadata = grpc::Metadata::new();
        metadata.add(grpc::MetadataKey::from("macaroon"), macaroon);
        metadata
    }
}
