use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::io::Result;
use httpbis::ClientTlsOption;

/// Represents bytes of the certificate
/// could be used to create `grpc::Client`
pub struct TLSCertificate {
    raw: tls_api::Certificate,
}

impl TLSCertificate {
    /// Reads the certificate in the pem format (other formats might work too, not tested)
    /// from a file at the path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
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

    /// Creates the tls using this certificate
    pub fn into_tls(self, host: &str) -> tls_api::Result<ClientTlsOption<tls_api_native_tls::TlsConnector>> {
        use tls_api::TlsConnectorBuilder as _;

        let mut builder = <tls_api_native_tls::TlsConnector as tls_api::TlsConnector>::builder()?;
        builder.add_root_certificate(self.raw)?;
        let connector = builder.build()?;
        Ok(ClientTlsOption::Tls(host.to_owned(), Arc::new(connector)))
    }
}
